use crate::{
    beacon::get_beacon,
    blockchain::{
        address::Address,
        block::{Block, NUMBER_OF_TRANSACTIONS_PER_BLOCK},
        transaction::Transaction,
    },
    state::State,
};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum Event {
    AddTransaction(Address, u64),
    CompletedMineBlock(Block),
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum Effect {
    MineBlock(Vec<Transaction>),
}

pub fn update(event: Event, state: State) -> (State, Vec<Effect>) {
    match event {
        Event::AddTransaction(address, amount) => {
            if let Ok(transaction) = Transaction::new_with_creating_signature(
                &state.address,
                &address,
                amount,
                &state.secret_key,
            ) {
                let new_transactions: Vec<Transaction> = state
                    .transactions
                    .into_iter()
                    .chain([transaction])
                    .collect();
                return if new_transactions.len() >= NUMBER_OF_TRANSACTIONS_PER_BLOCK {
                    (
                        State {
                            transactions: Vec::new(),
                            ..state
                        },
                        vec![Effect::MineBlock(new_transactions)],
                    )
                } else {
                    (
                        State {
                            transactions: new_transactions,
                            ..state
                        },
                        Vec::new(),
                    )
                };
            };
        }
        Event::CompletedMineBlock(new_block) => {
            let new_state = State {
                chain: state.chain.add_block(new_block),
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
            let Some(beacon) = get_beacon() else {
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
