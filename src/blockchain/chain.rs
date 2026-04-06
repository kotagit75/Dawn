use openssl::error::ErrorStack;
use serde::{Deserialize, Serialize};

use crate::{
    beacon::{Beacon, get_beacon},
    blockchain::{
        address::Address,
        block::{Block, genesis_block, solve_block_vdf},
        transaction::{Transaction, UnspentTransaction, get_transaction_out},
    },
    util::key::SK,
};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Chain {
    blocks: Vec<Block>,
}

impl Chain {
    pub fn new() -> Self {
        Self {
            blocks: vec![genesis_block()],
        }
    }

    pub fn get_latest_block(&self) -> Block {
        match self.blocks.last() {
            Some(block) => block.clone(),
            None => genesis_block(),
        }
    }

    pub fn generate_next_block(
        &self,
        sk: &SK,
        issuer: &Address,
        beacon: Beacon,
        transactions: Vec<Transaction>,
    ) -> Result<Block, ErrorStack> {
        let previous_block: Block = self.get_latest_block();
        let next_index: u64 = previous_block.index + 1;
        let next_timestamp: i64 = chrono::Utc::now().timestamp_millis();
        let vdf_solution = solve_block_vdf(
            next_index,
            next_timestamp,
            &transactions,
            beacon.clone(),
            issuer,
            previous_block.hash,
        )
        .unwrap();
        Block::new_with_creating_signature(
            next_index,
            next_timestamp,
            transactions,
            beacon,
            vdf_solution,
            issuer,
            previous_block.hash,
            sk,
        )
    }

    pub fn is_valid(&self) -> bool {
        let is_valid_genesis_block = self.blocks.first().cloned() == Some(genesis_block());
        let is_valid_chain = self
            .blocks
            .windows(2)
            .all(|windows| is_valid_new_block(&windows[0], &windows[1]));
        is_valid_genesis_block && is_valid_chain
    }

    pub fn replace(&self, new_chain: Chain) -> Self {
        if new_chain.is_valid() && new_chain.blocks.len() > self.blocks.len() {
            Self {
                blocks: new_chain.blocks,
            }
        } else {
            self.clone()
        }
    }

    pub fn add_block(&self, block: Block, generated_now: bool) -> Self {
        if generated_now && get_beacon(&self.get_beacon_history()) != Some(block.beacon.clone()) {
            return self.clone();
        }
        if is_valid_new_block(&block, &self.get_latest_block()) {
            Self {
                blocks: self
                    .blocks
                    .iter()
                    .chain(std::iter::once(&block))
                    .cloned()
                    .collect(),
            }
        } else {
            self.clone()
        }
    }

    pub fn get_unspent_transactions(&self) -> (Vec<UnspentTransaction>, u64 /*new id */) {
        self.blocks.iter().fold((Vec::new(), 1), |acc, block| {
            block.get_unspent_transactions(acc)
        })
    }

    pub fn generate_transaction(
        &self,
        sender: &Address,
        recipient: &Address,
        amount: u64,
        secret_key: &SK,
    ) -> Option<Result<Transaction, ErrorStack>> {
        let (unspent_transactions, _) = self.get_unspent_transactions();
        unspent_transactions
            .iter()
            .find(|tx| &tx.address == sender)
            .map(|unspent_transaction| {
                Transaction::new_with_creating_signature(
                    sender,
                    get_transaction_out(sender, recipient, amount, unspent_transaction.amount),
                    unspent_transaction.id,
                    secret_key,
                )
            })
    }

    pub fn get_balance(&self, address: &Address) -> u64 {
        let (unspent_transactions, _) = self.get_unspent_transactions();
        unspent_transactions
            .iter()
            .filter(|tx| &tx.address == address)
            .map(|tx| tx.amount)
            .sum()
    }

    pub fn get_beacon_history(&self) -> Vec<Beacon> {
        self.blocks
            .iter()
            .map(|block| block.beacon.clone())
            .collect()
    }
}

pub fn is_valid_new_block(block: &Block, previous_block: &Block) -> bool {
    block.index == previous_block.index + 1
        && block.timestamp > previous_block.timestamp
        && block.previous_hash == previous_block.hash
        && block.calculate_hash() == block.hash
}
