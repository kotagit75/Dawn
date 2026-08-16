pub mod command;
pub mod dummy;

use std::{future::Future, pin::Pin};

use crate::BeaconLocation;

pub trait BeaconProvider: Send + Sync {
    fn fetch_temperature<'a>(
        &'a mut self,
        location: &'a BeaconLocation,
        timestamp: i64,
    ) -> Pin<Box<dyn Future<Output = Option<i32>> + Send + 'a>>;
}
