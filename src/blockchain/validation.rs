use std::sync::LazyLock;

use regex::Regex;

use crate::{
    beacon::{BeaconCache, BeaconKey, is_valid_beacon},
    blockchain::{
        address::Address,
        block::{Block, MAX_TRANSACTIONS_PER_BLOCK, genesis_block},
        chain::{CHECKPOINT_DEPTH, Chain},
        coinbase::{coinbase_address, coinbase_amount},
        transaction::Transaction,
        utxo::UnspentTransaction,
    },
};

static ADDRESS_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^[a-fA-F0-9]+$").unwrap());

pub fn is_valid_address(address: &Address) -> bool {
    address.der.starts_with("30") && ADDRESS_RE.is_match(&address.der)
}

impl Transaction {
    pub fn is_valid(&self, unspent_transactions: &[UnspentTransaction]) -> bool {
        self.verify_signature()
            && self.total_amount() > 0
            && is_valid_address(&self.sender)
            && self
                .out
                .iter()
                .all(|txout| is_valid_address(&txout.address))
            && self.calc_total_input_amount(unspent_transactions) == self.total_amount()
            && self
                .tx_in
                .iter()
                .all(|tx_in| tx_in.get_amount(unspent_transactions).is_some())
    }
}

pub fn is_valid_coinbase_transaction(transaction: &Transaction, block_height: u64) -> bool {
    transaction.sender == coinbase_address()
        && transaction.tx_in.is_empty()
        && transaction.out.len() == 1
        && transaction.out[0].amount == coinbase_amount(block_height)
        && transaction
            .out
            .iter()
            .all(|txout| is_valid_address(&txout.address))
}

impl Block {
    pub fn is_valid(&self, unspent_transactions: &[UnspentTransaction]) -> bool {
        if self.transactions.len() > MAX_TRANSACTIONS_PER_BLOCK {
            return false;
        }
        if let Some((coinbase, normal)) = self.transactions.split_first() {
            self.verify_signature()
                && self.verify_vdf_solution()
                && is_valid_coinbase_transaction(coinbase, self.get_block_height())
                && normal.iter().all(|t| t.is_valid(unspent_transactions))
        } else {
            false
        }
    }
}

impl Chain {
    pub fn is_valid(&self, cache: &dyn BeaconCache) -> bool {
        let is_valid_genesis_block = self.blocks.first().cloned() == Some(genesis_block());
        let is_valid_chain = self.blocks.windows(2).all(|windows| {
            is_valid_new_block(
                &windows[1],
                &windows[0],
                &self.get_unspent_transactions().0,
                self.get_block_depth(&windows[1]),
                cache,
            )
        });
        is_valid_genesis_block && is_valid_chain
    }
}

pub fn is_valid_new_block(
    block: &Block,
    previous_block: &Block,
    unspent_transactions: &[UnspentTransaction],
    block_depth: usize,
    cache: &dyn BeaconCache,
) -> bool {
    let beacon_ok = if block_depth > CHECKPOINT_DEPTH {
        true
    } else {
        let Some(beacon) = cache.get(&BeaconKey::new(&previous_block.hash, block.timestamp)) else {
            return false;
        };
        is_valid_beacon(&beacon, &block.beacon)
    };
    block.index == previous_block.index + 1
        && block.timestamp > previous_block.timestamp
        && block.previous_hash == previous_block.hash
        && block.calculate_hash() == block.hash
        && block.is_valid(unspent_transactions)
        && beacon_ok
}
