use crate::{
    beacon::{BeaconCache, prefetch_beacon},
    blockchain::chain::Chain,
    p2p::{P2PMessage, Peer},
    state::State,
    update::{
        beacon::prefetch_chain_beacons,
        effect::{Effect, map_effect},
    },
};

pub async fn handle_p2p_message(
    state: State,
    beacon_cache: &dyn BeaconCache,
    peer_option: Option<Peer>,
    message: P2PMessage,
) -> (State, Effect) {
    match message {
        P2PMessage::QueryAll => {
            let chain = state.chain.blocks.clone();
            return (
                state,
                Effect::Broadcast(P2PMessage::ResponseBlockChain(chain)),
            );
        }
        P2PMessage::QueryLatest => {
            let blocks = vec![state.chain.get_latest_block()];
            return (
                state,
                Effect::Broadcast(P2PMessage::ResponseBlockChain(blocks)),
            );
        }
        P2PMessage::ResponseBlockChain(blocks) => {
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
                    let (new_chain, changed) = state.chain.add_block(
                        received_latest_block.clone(),
                        false,
                        true,
                        beacon_cache,
                    );
                    if changed {
                        info!("added block: {:?}", received_latest_block);
                    }
                    return (
                        State {
                            chain: new_chain,
                            ..state
                        },
                        map_effect(
                            || {
                                Effect::Broadcast(P2PMessage::ResponseBlockChain(vec![
                                    received_latest_block.clone(),
                                ]))
                            },
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
            return (state, Effect::None);
        }
        P2PMessage::QueryTransactions => {
            return (
                state.clone(),
                Effect::Broadcast(P2PMessage::ResponseTransactions(state.transactions.clone())),
            );
        }
        P2PMessage::ResponseTransactions(transactions) => {
            let (state, changed) =
                transactions
                    .iter()
                    .fold((state, false), |(state, changed), transaction| {
                        let (state, changed_) = state.add_transaction(transaction);
                        (state, changed || changed_)
                    });
            return (
                state.clone(),
                map_effect(
                    || Effect::Broadcast(P2PMessage::ResponseTransactions(state.transactions)),
                    changed,
                ),
            );
        }
        P2PMessage::QueryPeers => {
            return (
                match peer_option {
                    Some(peer) => state.add_peer(&peer).0,
                    None => state.clone(),
                },
                Effect::Broadcast(P2PMessage::ResponsePeers(state.peers.clone())),
            );
        }
        P2PMessage::ResponsePeers(peers) => {
            let (state, changed) = state.add_peers(&peers);
            return (
                state.clone(),
                map_effect(
                    || Effect::Broadcast(P2PMessage::ResponsePeers(state.peers)),
                    changed,
                ),
            );
        }
    }
}
