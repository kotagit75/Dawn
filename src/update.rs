use std::vec;

use crate::{
    beacon::get_beacon,
    blockchain::{
        address::Address,
        block::Block,
        chain::Chain,
        transaction::{Transaction, coinbase_transaction},
    },
    p2p::{P2PMessage, Peer, broadcast},
    state::State,
};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum Event {
    AddPeer(Peer),
    AddTransaction(Address, u64),
    MineBlock,
    CompletedMineBlock(Block),
    P2PMessage(P2PMessage),
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum Effect {
    MineBlock(Vec<Transaction>),
    BroadcastQueryAll,
    BroadcastResponseBlocks(Vec<Block>),
    BroadcastResponseTransactions(Vec<Transaction>),
}

pub fn update(event: Event, state: State) -> (State, Vec<Effect>) {
    match event {
        Event::AddPeer(peer) => {
            let new_peers: Vec<Peer> = state.peers.into_iter().chain([peer]).collect();
            return (
                State {
                    peers: new_peers,
                    ..state
                },
                Vec::new(),
            );
        }
        Event::AddTransaction(recipient, amount) => {
            if let Ok(Some(transaction)) = state.chain.generate_transaction(
                &state.address,
                &recipient,
                amount,
                &state.secret_key,
            ) {
                let (state, changed) = state.add_to_transaction(&transaction);
                return (state, {
                    if changed {
                        vec![Effect::BroadcastResponseTransactions(vec![
                            transaction.clone(),
                        ])]
                    } else {
                        Vec::new()
                    }
                });
            };
        }
        Event::MineBlock => {
            let coinbase = coinbase_transaction(&state.address);
            let blocks_for_mine: Vec<Transaction> = [coinbase]
                .iter()
                .chain(&state.transactions)
                .cloned()
                .collect();
            return (
                State {
                    transactions: Vec::new(),
                    ..state
                },
                vec![Effect::MineBlock(blocks_for_mine)],
            );
        }
        Event::CompletedMineBlock(new_block) => {
            let (chain, changed) = state.chain.add_block(new_block.clone(), true);
            let new_state = State {
                chain,
                transactions: Vec::new(),
                ..state
            };
            return (new_state, {
                if changed {
                    vec![Effect::BroadcastResponseBlocks(vec![new_block])]
                } else {
                    Vec::new()
                }
            });
        }
        Event::P2PMessage(P2PMessage::QueryAll) => {
            return (
                state.clone(),
                vec![Effect::BroadcastResponseBlocks(state.chain.blocks.clone())],
            );
        }
        Event::P2PMessage(P2PMessage::QueryLatest) => {
            return (
                state.clone(),
                vec![Effect::BroadcastResponseBlocks(vec![
                    state.chain.get_latest_block(),
                ])],
            );
        }
        Event::P2PMessage(P2PMessage::ResponseBlockChain(blocks)) => {
            let Some(received_lastest_block) = blocks.last() else {
                return (state, Vec::new());
            };
            let held_lastest_block = state.chain.get_latest_block();
            if received_lastest_block.index > held_lastest_block.index {
                if received_lastest_block.previous_hash == held_lastest_block.hash {
                    let (new_chain, changed) =
                        state.chain.add_block(received_lastest_block.clone(), true);
                    return (
                        State {
                            chain: new_chain,
                            ..state
                        },
                        {
                            if changed {
                                vec![Effect::BroadcastResponseBlocks(vec![
                                    received_lastest_block.clone(),
                                ])]
                            } else {
                                Vec::new()
                            }
                        },
                    );
                } else if blocks.len() == 1 {
                    return (state, vec![Effect::BroadcastQueryAll]);
                } else {
                    return (
                        State {
                            chain: state.chain.replace(Chain { blocks }),
                            ..state
                        },
                        Vec::new(),
                    );
                }
            }
        }
        Event::P2PMessage(P2PMessage::QueryTransactions) => {
            return (
                state.clone(),
                vec![Effect::BroadcastResponseTransactions(
                    state.transactions.clone(),
                )],
            );
        }
        Event::P2PMessage(P2PMessage::ResponseTransactions(transactions)) => {
            let (state, changed) =
                transactions
                    .iter()
                    .fold((state, false), |(state, changed), transaction| {
                        let (state, changed_) = state.add_to_transaction(transaction);
                        (state, changed || changed_)
                    });
            return (state.clone(), {
                if changed {
                    vec![Effect::BroadcastResponseTransactions(
                        state.transactions.clone(),
                    )]
                } else {
                    Vec::new()
                }
            });
        }
    }
    (state, Vec::new())
}

pub async fn run_effect(state: State, event_tx: mpsc::Sender<Event>, effect: Effect) {
    match effect {
        Effect::MineBlock(transactions) => {
            let Some(beacon) = get_beacon(
                &state.chain.get_beacon_history(),
                &state.chain.get_latest_block().hash,
            ) else {
                return;
            };
            let Ok(event) = state
                .chain
                .generate_next_block(&state.secret_key, &state.address, beacon, transactions)
                .map(|block| Event::CompletedMineBlock(block))
            else {
                return;
            };
            let _ = event_tx.send(event).await;
        }
        Effect::BroadcastResponseBlocks(blocks) => {
            broadcast(&state.peers, &P2PMessage::ResponseBlockChain(blocks)).await;
        }
        Effect::BroadcastQueryAll => {
            broadcast(&state.peers, &P2PMessage::QueryAll).await;
        }
        Effect::BroadcastResponseTransactions(transactions) => {
            broadcast(
                &state.peers,
                &P2PMessage::ResponseTransactions(transactions),
            )
            .await;
        }
    }
}
