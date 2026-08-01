use crate::{
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

pub async fn handle_completed_mine_block(state: State, new_block: Block) -> (State, Effect) {
    let (chain, changed) = state.chain.add_block(new_block.clone(), None);
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
