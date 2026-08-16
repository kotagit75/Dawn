use std::{collections::HashMap, sync::Mutex};

use btfy_util::hash::Hashed;

use crate::Beacon;

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
