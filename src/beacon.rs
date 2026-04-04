use std::{env::current_dir, process::Command};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Beacon {
    pub value: i32,
}

const BEACON_COMMAND: &str = "./beacon.sh";

pub fn get_beacon() -> Option<Beacon> {
    match current_dir() {
        Ok(x) => Command::new(BEACON_COMMAND)
            .current_dir(x)
            .status()
            .ok()
            .and_then(|status| status.code()),
        Err(_) => None,
    }
    .map(|value| Beacon { value })
}
