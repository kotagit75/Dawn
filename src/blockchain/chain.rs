use openssl::error::ErrorStack;
use serde::{Deserialize, Serialize};

use crate::{
    beacon::Beacon,
    blockchain::{
        block::{Block, genesis_block},
        transaction::Transaction,
    },
    util::key::SK,
};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
struct Chain {
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
        beacon: Beacon,
        transactions: Vec<Transaction>,
    ) -> Result<Block, ErrorStack> {
        let previous_block: Block = self.get_latest_block();
        let next_index: u64 = previous_block.index + 1;
        let next_timestamp: i64 = chrono::Utc::now().timestamp_millis();
        Block::new_with_creating_signature(
            next_index,
            next_timestamp,
            transactions,
            beacon,
            vec![],
            &previous_block.issuer,
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

    pub fn add_block(&self, block: Block) -> Self {
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
}

pub fn is_valid_new_block(block: &Block, previous_block: &Block) -> bool {
    block.index == previous_block.index + 1
        && block.timestamp > previous_block.timestamp
        && block.previous_hash == previous_block.hash
        && block.calculate_hash() == block.hash
}
