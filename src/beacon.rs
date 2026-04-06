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
        (33.5901838, 130.4016888),
        (41.7686961, 140.7290599),
        (35.4436739, 139.6379639),
        (35.011564, 135.7681489),
        (38.268195, 140.869418),
        (34.6900806, 135.1956311),
        (35.1814506, 136.9065571),
        (34.3852894, 132.4553055),
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
