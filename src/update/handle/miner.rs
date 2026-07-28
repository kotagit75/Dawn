use crate::{
    beacon::{BeaconCache, prefetch_beacon},
    blockchain::block::{Block, MAX_TRANSACTIONS_PER_BLOCK},
    p2p::P2PMessage,
    state::State,
    update::effect::{Effect, when_changed},
};

pub fn handle_mine_block(state: State) -> (State, Effect) {
    let mut sorted_transactions: Vec<_> = state.transactions.clone();
    sorted_transactions.sort_by_key(|tx| tx.fee);
    sorted_transactions.reverse();
    let (transactions_to_mine, remaining_transactions) = sorted_transactions.split_at(
        std::cmp::min(MAX_TRANSACTIONS_PER_BLOCK, sorted_transactions.len()),
    );

    (
        State {
            transactions: remaining_transactions.to_vec(),
            ..state
        },
        Effect::MineBlock(transactions_to_mine.to_vec()),
    )
}

pub async fn handle_completed_mine_block(
    state: State,
    beacon_cache: &dyn BeaconCache,
    new_block: Block,
) -> (State, Effect) {
    let _ = prefetch_beacon(
        beacon_cache,
        &state.chain.get_latest_block().hash,
        new_block.timestamp,
    )
    .await;
    let (chain, changed) = state
        .chain
        .add_block(new_block.clone(), true, true, beacon_cache);
    let state = State { chain, ..state };

    if changed {
        info!("added next block to chain");
    } else {
        error!("failed to add next block");
    }

    (
        state,
        when_changed(
            Effect::Broadcast(P2PMessage::ResponseBlockChain(vec![new_block])),
            changed,
        ),
    )
}
