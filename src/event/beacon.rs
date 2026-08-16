use crate::{
    beacon::{BeaconCache, prefetch_beacon, provider::BeaconProvider},
    blockchain::{block::Block, chain::CHECKPOINT_DEPTH},
};

pub async fn prefetch_chain_beacons<T: BeaconProvider>(
    beacon_provider: &mut T,
    cache: &dyn BeaconCache,
    blocks: &[Block],
) {
    if blocks.len() < 2 {
        return;
    }
    let start = blocks.len().saturating_sub(CHECKPOINT_DEPTH + 1);
    for window in blocks[start..].windows(2) {
        prefetch_beacon(beacon_provider, cache, &window[0].hash, window[1].timestamp).await;
    }
}
