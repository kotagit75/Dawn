use crate::{
    beacon::get_beacon,
    blockchain::{
        address::Address,
        block::Block,
        transaction::{Transaction, coinbase_transaction},
    },
    state::State,
};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum Event {
    AddTransaction(Address, u64),
    MineBlock,
    CompletedMineBlock(Block),
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum Effect {
    MineBlock(Vec<Transaction>),
}

pub fn update(event: Event, state: State) -> (State, Vec<Effect>) {
    match event {
        Event::AddTransaction(recipient, amount) => {
            if let Some(Ok(transaction)) = state.chain.generate_transaction(
                &state.address,
                &recipient,
                amount,
                &state.secret_key,
            ) {
                let new_transactions: Vec<Transaction> = state
                    .transactions
                    .into_iter()
                    .chain([transaction])
                    .collect();
                return (
                    State {
                        transactions: new_transactions,
                        ..state
                    },
                    Vec::new(),
                );
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
            let new_state = State {
                chain: state.chain.add_block(new_block, true),
                transactions: Vec::new(),
                ..state
            };
            return (new_state, Vec::new());
        }
    }
    (state, Vec::new())
}

pub async fn run_effect(state: State, event_tx: mpsc::Sender<Event>, effect: Effect) {
    match effect {
        Effect::MineBlock(transactions) => {
            let Some(beacon) = get_beacon(&state.chain.get_beacon_history()) else {
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
    }
}
