use bitcode::{Decode, Encode};
use serde::{Deserialize, Serialize};

use crate::{
    beacon::{
        cache::{BeaconCache, BeaconKey},
        location::{BEACON_LOCATIONS, BeaconLocation},
        provider::BeaconProvider,
    },
    util::{hash::Hashed, progressbar::create_progress_bar},
};

pub mod cache;
pub mod location;
pub mod provider;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Encode, Decode)]
pub struct Beacon {
    pub values: Vec<i32>,
}

async fn fetch_temperature<T: BeaconProvider>(
    provider: &mut T,
    location: &BeaconLocation,
    timestamp: i64,
) -> Option<i32> {
    provider.fetch_temperature(location, timestamp).await
}

fn choose_locations(latest_block_hash: &Hashed) -> Vec<BeaconLocation> {
    let len = BEACON_LOCATIONS.len();
    if len == 0 {
        return Vec::new();
    }
    latest_block_hash
        .iter()
        .flat_map(|i| BEACON_LOCATIONS.get((*i as usize) % len))
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
