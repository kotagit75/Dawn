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
        (35.452405, 139.643815),
        (33.586608, 130.437317),
        (43.079165, 141.336388),
        (38.253449, 140.856765),
        (35.060372, 135.785899),
        (34.075282, 134.554346),
        (41.797463, 140.757250),
        (40.821890, 140.446748),
        (34.410487, 133.196552),
        (36.247005, 137.954883),
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
