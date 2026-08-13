use serde::{Deserialize, Serialize};

use crate::{
    blockchain::{
        address::Address,
        chain::Chain,
        transaction::Transaction,
        utxo::{transaction_to_unspent_ids, transactions_to_unspent_ids},
    },
    p2p::Peer,
    util::key::SK,
};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct State {
    pub secret_key: SK,
    pub address: Address,
    pub chain: Chain,
    pub transactions: Vec<Transaction>,
    pub peers: Vec<Peer>,
}

const MAX_PEERS: usize = 64;

impl State {
    pub fn new(secret_key: SK, chain: Chain) -> Self {
        let address = secret_key.to_pk();
        Self {
            secret_key,
            address,
            chain,
            transactions: Vec::new(),
            peers: Vec::new(),
        }
    }

    fn add_transaction_to_pool_without_validation(&mut self, transaction: &Transaction) {
        self.transactions.push(transaction.clone());
    }

    pub fn add_transaction_to_pool(&mut self, transaction: &Transaction) -> bool {
        let tx_in_ids = transaction_to_unspent_ids(transaction);
        let state_tx_in_ids = transactions_to_unspent_ids(&self.transactions);

        let is_valid = transaction.is_valid(&self.chain.get_unspent_transactions().0);
        let inputs_exist =
            self.chain.find_unspent_transactions(&tx_in_ids).len() == tx_in_ids.len();
        let double_spent_in_pool = tx_in_ids.iter().any(|id| state_tx_in_ids.contains(id));

        if !is_valid || !inputs_exist || double_spent_in_pool {
            return false;
        }

        self.add_transaction_to_pool_without_validation(transaction);
        true
    }

    pub fn add_peers(&mut self, new_peers: &[Peer]) -> bool {
        let mut added = false;
        for new_peer in new_peers {
            if !self.peers.contains(new_peer) && self.peers.len() <= MAX_PEERS {
                self.peers.push(new_peer.clone());
                info!("added peer: {}", new_peer.addr);
                added = true;
            } else {
                error!("peer already exists: {}", new_peer.addr);
            }
        }
        added
    }
}
