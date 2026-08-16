use crate::beacon::{BeaconLocation, provider::BeaconProvider};
use std::{future::Future, pin::Pin};

pub struct DummyBeaconProvider;

impl BeaconProvider for DummyBeaconProvider {
    fn fetch_temperature<'a>(
        &'a mut self,
        _location: &'a BeaconLocation,
        _timestamp: i64,
    ) -> Pin<Box<dyn Future<Output = Option<i32>> + Send + 'a>> {
        Box::pin(async move { Some(10) })
    }
}
