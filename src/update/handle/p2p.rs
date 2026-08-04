use crate::{
    beacon::{BeaconCache, prefetch_beacon},
    blockchain::{block::Block, chain::Chain, transaction::Transaction},
    p2p::{P2PMessage, Peer},
    state::{State, add_peers, add_transaction_to_pool},
    update::{
        beacon::prefetch_chain_beacons,
        effect::{Effect, when_changed},
    },
};

pub async fn handle_p2p_message(
    state: State,
    beacon_cache: &dyn BeaconCache,
    peer_option: Option<Peer>,
    message: P2PMessage,
) -> (State, Effect) {
    match message {
        P2PMessage::QueryAll => handle_query_all(state),
        P2PMessage::QueryLatest => handle_query_latest(state),
        P2PMessage::ResponseBlockChain(blocks) => {
            handle_response_block_chain(state, blocks, beacon_cache).await
        }
        P2PMessage::QueryTransactions => handle_query_transactions(state),
        P2PMessage::ResponseTransactions(transactions) => {
            handle_response_transactions(state, transactions)
        }
        P2PMessage::QueryPeers => handle_query_peers(state, peer_option),
        P2PMessage::ResponsePeers(peers) => handle_response_peers(state, peers),
    }
}

fn handle_query_all(state: State) -> (State, Effect) {
    let chain = state.chain.blocks.clone();
    (
        state,
        Effect::Broadcast(P2PMessage::ResponseBlockChain(chain)),
    )
}

fn handle_query_latest(state: State) -> (State, Effect) {
    let blocks = vec![state.chain.get_latest_block()];
    (
        state,
        Effect::Broadcast(P2PMessage::ResponseBlockChain(blocks)),
    )
}

async fn handle_response_block_chain(
    state: State,
    blocks: Vec<Block>,
    beacon_cache: &dyn BeaconCache,
) -> (State, Effect) {
    let Some(received_latest_block) = blocks.last() else {
        return (state, Effect::None);
    };
    let held_latest_block = state.chain.get_latest_block();
    if received_latest_block.index > held_latest_block.index {
        if received_latest_block.previous_hash == held_latest_block.hash {
            let _ = prefetch_beacon(
                beacon_cache,
                &held_latest_block.hash,
                received_latest_block.timestamp,
            )
            .await;
            let (new_chain, changed) = state
                .chain
                .add_block(received_latest_block.clone(), Some(beacon_cache));
            if changed {
                info!("added block to chain");
            }
            return (
                State {
                    chain: new_chain,
                    ..state
                },
                when_changed(
                    Effect::Broadcast(P2PMessage::ResponseBlockChain(vec![
                        received_latest_block.clone(),
                    ])),
                    changed,
                ),
            );
        } else if blocks.len() == 1 {
            return (state, Effect::Broadcast(P2PMessage::QueryAll));
        } else {
            prefetch_chain_beacons(beacon_cache, &blocks).await;
            info!("replacing chain with {} blocks", blocks.len());
            return (
                State {
                    chain: state.chain.replace(Chain { blocks }, beacon_cache),
                    ..state
                },
                Effect::None,
            );
        }
    }
    (state, Effect::None)
}

fn handle_query_transactions(state: State) -> (State, Effect) {
    (
        state.clone(),
        Effect::Broadcast(P2PMessage::ResponseTransactions(state.transactions.clone())),
    )
}

fn handle_response_transactions(state: State, transactions: Vec<Transaction>) -> (State, Effect) {
    let (state, changed) =
        transactions
            .iter()
            .fold((state, false), |(state, changed), transaction| {
                let (state, changed_) = add_transaction_to_pool(state, transaction);
                (state, changed || changed_)
            });
    (
        state.clone(),
        when_changed(
            Effect::Broadcast(P2PMessage::ResponseTransactions(state.transactions)),
            changed,
        ),
    )
}

fn handle_query_peers(state: State, peer_option: Option<Peer>) -> (State, Effect) {
    (
        peer_option
            .map(|peer| add_peers(state.clone(), &[peer]).0)
            .unwrap_or(state.clone()),
        Effect::Broadcast(P2PMessage::ResponsePeers(state.peers.clone())),
    )
}

fn handle_response_peers(state: State, peers: Vec<Peer>) -> (State, Effect) {
    let (state, changed) = add_peers(state, &peers);
    (
        state.clone(),
        when_changed(
            Effect::Broadcast(P2PMessage::ResponsePeers(state.peers)),
            changed,
        ),
    )
}
