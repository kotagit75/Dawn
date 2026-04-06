use std::{env::current_dir, process::Command};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Beacon {
    pub value: i32,
}

fn get_temperature(lat: f64, lon: f64) -> Option<i32> {
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
    let positions = [
        (40.782514, -73.965446),
        (36.2048, 138.2529),
        (51.476856, -0.0004847),
    ];
    let sum: i32 = positions
        .map(|pos| get_temperature(pos.0, pos.1))
        .iter()
        .flatten()
        .sum();
    Some(Beacon {
        value: sum + history.iter().map(|b| b.value).sum::<i32>(),
    })
}
