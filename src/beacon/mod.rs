use bitcode::{Decode, Encode};
use geojson::{FeatureCollection, GeometryValue};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::{LazyLock, Mutex},
};

use crate::{
    beacon::provider::BeaconProvider,
    util::{hash::Hashed, progressbar::create_progress_bar},
};

pub mod provider;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Encode, Decode)]
pub struct Beacon {
    pub values: Vec<i32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BeaconKey {
    pub latest_block_hash: Hashed,
    pub timestamp: i64,
}

impl BeaconKey {
    pub fn new(latest_block_hash: &Hashed, timestamp: i64) -> Self {
        Self {
            latest_block_hash: *latest_block_hash,
            timestamp,
        }
    }
}

pub trait BeaconCache: Send + Sync {
    fn get(&self, key: &BeaconKey) -> Option<Beacon>;
    fn insert(&self, key: BeaconKey, beacon: Beacon);
}

#[derive(Default)]
pub struct InMemoryBeaconCache {
    inner: Mutex<HashMap<BeaconKey, Beacon>>,
}

impl InMemoryBeaconCache {
    pub fn new() -> Self {
        Self::default()
    }
}

impl BeaconCache for InMemoryBeaconCache {
    fn get(&self, key: &BeaconKey) -> Option<Beacon> {
        self.inner
            .lock()
            .expect("beacon cache lock poisoned")
            .get(key)
            .cloned()
    }

    fn insert(&self, key: BeaconKey, beacon: Beacon) {
        self.inner
            .lock()
            .expect("beacon cache lock poisoned")
            .insert(key, beacon);
    }
}

#[derive(Debug, Clone)]
pub struct BeaconLocation {
    lat: f64,
    lon: f64,
    icao_code: String,
}
const LOCATIONS_GEOJSON: &str = include_str!("locations.geojson");
static LOCATIONS_LOCATIONS: LazyLock<Vec<BeaconLocation>> = LazyLock::new(|| {
    let Ok(collection) = LOCATIONS_GEOJSON.parse::<FeatureCollection>() else {
        return Vec::new();
    };
    let features = collection.features;

    let mut result = Vec::new();
    for feature in features {
        let Some(geometry) = feature.geometry else {
            continue;
        };
        let Some(properties) = feature.properties else {
            continue;
        };
        let (lat, lon) = match geometry.value {
            GeometryValue::Point { coordinates } => (coordinates[1], coordinates[0]),
            _ => continue,
        };

        let icao_code = match properties.get("icao_code") {
            Some(code) => code.as_str(),
            None => continue,
        };

        let Some(icao_code) = icao_code else {
            continue;
        };

        result.push(BeaconLocation {
            lat,
            lon,
            icao_code: icao_code.to_string(),
        });
    }
    result
});

async fn fetch_temperature<T: BeaconProvider>(
    provider: &mut T,
    location: &BeaconLocation,
    timestamp: i64,
) -> Option<i32> {
    let result = provider.fetch_temperature(location, timestamp).await;
    result
}

fn choose_locations(latest_block_hash: &Hashed) -> Vec<BeaconLocation> {
    let len = LOCATIONS_LOCATIONS.len();
    if len == 0 {
        return Vec::new();
    }
    latest_block_hash
        .iter()
        .flat_map(|i| LOCATIONS_LOCATIONS.get((*i as usize) % len))
        .cloned()
        .collect()
}

pub async fn fetch_beacon<T: BeaconProvider>(
    provider: &mut T,
    latest_block_hash: &Hashed,
    timestamp: i64,
) -> Option<Beacon> {
    let locations: Vec<_> = choose_locations(latest_block_hash);
    let mut temperatures: Vec<i32> = Vec::new();

    let pb = create_progress_bar(locations.len() as u64);
    pb.set_message("fetching beacon");

    for (i, location) in locations.iter().enumerate() {
        if let Some(temp) = fetch_temperature(provider, location, timestamp).await {
            temperatures.push(temp);
            pb.inc(1);
        } else {
            pb.abandon_with_message(format!("failed to fetch temperature for location {}", i));
            break;
        }
    }
    if temperatures.len() != locations.len() {
        return None;
    }
    pb.abandon_with_message("beacon fetched");
    Some(Beacon {
        values: temperatures,
    })
}

pub async fn prefetch_beacon<T: BeaconProvider>(
    provider: &mut T,
    cache: &dyn BeaconCache,
    latest_block_hash: &Hashed,
    timestamp: i64,
) -> bool {
    let key = BeaconKey::new(latest_block_hash, timestamp);
    if cache.get(&key).is_some() {
        return true;
    }
    let Some(beacon) = fetch_beacon(provider, latest_block_hash, timestamp).await else {
        return false;
    };
    cache.insert(key, beacon);
    true
}

pub fn is_valid_beacon(own_beacon: &Beacon, target_beacon: &Beacon) -> bool {
    own_beacon
        .values
        .iter()
        .zip(target_beacon.values.iter())
        .all(
            |(a, b)| (a - b).abs() <= 5, /* Allowable error is within 0.5 degrees Celsius.*/
        )
}
