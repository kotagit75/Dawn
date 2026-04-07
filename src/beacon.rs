use geojson::{FeatureCollection, GeometryValue};
use std::{env::current_dir, process::Command};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Beacon {
    pub value: i32,
}

fn get_temperature(lon: f64, lat: f64) -> Option<i32> {
    match current_dir() {
        Ok(x) => {
            let status = Command::new("beacon/temperature.sh")
                .arg(lat.to_string())
                .arg(lon.to_string())
                .current_dir(x)
                .status();
            status.ok().and_then(|status| status.code())
        }
        Err(_) => None,
    }
}

pub fn get_beacon(history: &[Beacon]) -> Option<Beacon> {
    let Ok(collection) = include_str!("beacon/target.geojson").parse::<FeatureCollection>() else {
        return None;
    };
    let locations: Vec<geojson::Position> = collection
        .features
        .iter()
        .map(|feature| feature.geometry.clone())
        .flatten()
        .map(|geometry| match geometry.value {
            GeometryValue::Point { coordinates } => Some(coordinates),
            _ => None,
        })
        .flatten()
        .collect();
    let sum: i32 = locations
        .iter()
        .map(|pos| get_temperature(pos[0], pos[1]))
        .flatten()
        .sum();
    Some(Beacon {
        value: sum + history.iter().map(|b| b.value).sum::<i32>(),
    })
}
