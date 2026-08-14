use futures::future::join_all;

use crate::{
    beacon::{BeaconCache, prefetch_beacon},
    blockchain::{block::Block, chain::CHECKPOINT_DEPTH},
};

pub async fn prefetch_chain_beacons(cache: &dyn BeaconCache, blocks: &[Block]) {
    if blocks.len() < 2 {
        return;
    }
    let start = blocks.len().saturating_sub(CHECKPOINT_DEPTH + 1);
    let tasks = blocks[start..]
        .windows(2)
        .map(|window| prefetch_beacon(cache, &window[0].hash, window[1].timestamp));
    let _ = join_all(tasks).await;
}
