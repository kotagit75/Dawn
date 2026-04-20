use geojson::{FeatureCollection, GeometryValue};
use std::{env::current_dir, process::Command};

use serde::{Deserialize, Serialize};

use crate::util::hash::Hashed;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Beacon {
    pub value: f32,
}

fn run_command_and_get_exit_code(command: &mut Command) -> Option<i32> {
    match current_dir() {
        Ok(x) => {
            let status = command.current_dir(x).status();
            status.ok().and_then(|status| status.code())
        }
        Err(_) => None,
    }
}

fn get_temperature(lon: f64, lat: f64) -> Option<f32> {
    let code = run_command_and_get_exit_code(
        Command::new("beacon/temperature.sh")
            .arg(lat.to_string())
            .arg(lon.to_string()),
    );
    code.map(|code| code as f32 / 10.0)
}

fn calc_locations(lastest_block_hash: &Hashed) -> Vec<geojson::Position> {
    let Ok(collection) = include_str!("beacon/target.geojson").parse::<FeatureCollection>() else {
        return Vec::new();
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
    let len = locations.len();
    lastest_block_hash
        .iter()
        .map(|i| (*i as usize) % len)
        .map(|i| locations.get(i))
        .flatten()
        .cloned()
        .collect()
}

pub fn get_beacon(history: &[Beacon], lastest_block_hash: &Hashed) -> Option<Beacon> {
    let locations: Vec<geojson::Position> = calc_locations(lastest_block_hash);
    let sum: f32 = locations
        .iter()
        .map(|pos| get_temperature(pos[0], pos[1]))
        .flatten()
        .sum();
    Some(Beacon {
        value: sum + history.iter().map(|b| b.value).sum::<f32>(),
    })
}

pub fn is_valid_beacon(
    target_beacon: &Beacon,
    history: &[Beacon],
    lastest_block_hash: &Hashed,
) -> bool {
    match get_beacon(history, lastest_block_hash) {
        Some(beacon) => (beacon.value - target_beacon.value).abs() <= 0.5,
        None => false,
    }
}
