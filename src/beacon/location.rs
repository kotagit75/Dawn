use std::sync::LazyLock;

use geojson::{FeatureCollection, GeometryValue};

#[derive(Debug, Clone)]
pub struct BeaconLocation {
    lat: f64,
    lon: f64,
    icao_code: String,
}

impl BeaconLocation {
    pub fn lat(&self) -> f64 {
        self.lat
    }

    pub fn lon(&self) -> f64 {
        self.lon
    }

    pub fn icao_code(&self) -> &str {
        &self.icao_code
    }
}

const LOCATIONS_GEOJSON: &str = include_str!("locations.geojson");
pub static BEACON_LOCATIONS: LazyLock<Vec<BeaconLocation>> = LazyLock::new(|| {
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
