use crate::blockchain::{
    address::Address, block::Block, chain::Chain, transaction::Transaction,
    utxo::UnspentTransaction,
};

impl Transaction {
    pub fn get_unspent_transactions(
        &self,
        (previous_unspent, first_id): (Vec<UnspentTransaction>, u64),
    ) -> (Vec<UnspentTransaction>, u64 /*new id */) {
        let (mut new_unspent, new_id) =
            self.out
                .iter()
                .fold((previous_unspent, first_id), |(mut acc, id), tx_out| {
                    let (unspent, new_id) = tx_out.to_unspent(id);
                    acc.push(unspent);
                    (acc, new_id)
                });
        new_unspent.retain(|unspent| {
            !self
                .tx_in
                .iter()
                .any(|tx_in| tx_in.unspent_id == unspent.id)
        });
        (new_unspent, new_id)
    }

    /*
     * This method calculates the total amount of the transaction input.
     */
    pub fn calc_total_input_amount(&self, unspent_transactions: &[UnspentTransaction]) -> u64 {
        self.tx_in
            .iter()
            .flat_map(|tx_in| tx_in.get_amount(unspent_transactions))
            .sum::<u64>()
    }

    pub fn fee_to_unspent_transaction(
        &self,
        miner: Address,
        (previous_unspent, first_id): (Vec<UnspentTransaction>, u64),
    ) -> (Vec<UnspentTransaction>, u64) {
        let fee_unspent = UnspentTransaction {
            id: first_id,
            address: miner,
            amount: self.fee,
        };
        (
            previous_unspent
                .iter()
                .chain([fee_unspent].iter())
                .cloned()
                .collect(),
            first_id + 1,
        )
    }
}

impl Block {
    pub fn get_unspent_transactions(
        &self,
        (previous_unspent, first_id): (Vec<UnspentTransaction>, u64),
    ) -> (Vec<UnspentTransaction>, u64 /*new id */) {
        self.transactions
            .iter()
            .fold((previous_unspent, first_id), |acc, tx| {
                tx.fee_to_unspent_transaction(self.issuer.clone(), tx.get_unspent_transactions(acc))
            })
    }
}

impl Chain {
    pub fn get_unspent_transactions(&self) -> (Vec<UnspentTransaction>, u64 /*new id */) {
        self.blocks.iter().fold((Vec::new(), 1), |acc, block| {
            block.get_unspent_transactions(acc)
        })
    }

    pub fn find_unspent_transaction(&self, unspent_id: u64) -> Option<UnspentTransaction> {
        let (unspent_transactions, _) = self.get_unspent_transactions();
        unspent_transactions
            .iter()
            .find(|unspent| unspent.id == unspent_id)
            .cloned()
    }

    pub fn find_unspent_transactions(&self, unspent_ids: &[u64]) -> Vec<UnspentTransaction> {
        let (unspent_transactions, _) = self.get_unspent_transactions();
        unspent_transactions
            .iter()
            .filter(|unspent| unspent_ids.contains(&unspent.id))
            .cloned()
            .collect()
    }

    pub fn filter_unspent_transactions_by_address(
        &self,
        address: &Address,
    ) -> Vec<UnspentTransaction> {
        self.get_unspent_transactions()
            .0
            .iter()
            .filter(|unspent| unspent.address == *address)
            .cloned()
            .collect()
    }

    pub fn get_balance(&self, address: &Address) -> u64 {
        let (unspent_transactions, _) = self.get_unspent_transactions();
        unspent_transactions
            .iter()
            .filter(|tx| &tx.address == address)
            .map(|tx| tx.amount)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use crate::blockchain::address::Address;
    use crate::blockchain::block::{Block, genesis_block};
    use crate::blockchain::coinbase::coinbase_transaction;
    use crate::util::key::{SK, generate_sk};
    use crate::util::signature::SignatureWrapper;

    fn keypair() -> (Address, SK) {
        let sk = generate_sk(512);
        let pk = sk.to_pk();
        (pk, sk)
    }

    fn dummy_block(
        prev: &Block,
        txs: Vec<crate::blockchain::transaction::Transaction>,
        beacon: i32,
    ) -> Block {
        Block {
            index: prev.index + 1,
            timestamp: prev.timestamp + 1,
            transactions: txs,
            beacon: crate::beacon::Beacon {
                values: vec![beacon],
            },
            vdf_solution: vec![],
            previous_hash: prev.hash,
            issuer: prev.issuer.clone(),
            signature: SignatureWrapper::default(),
            hash: [prev.index as u8 + 1; 32],
        }
    }

    fn chain_with_coinbase(miner: &Address) -> crate::blockchain::chain::Chain {
        let g = genesis_block();
        let b1 = dummy_block(&g, vec![coinbase_transaction(miner, 1)], 1);
        crate::blockchain::chain::Chain {
            blocks: vec![g, b1],
        }
    }

    #[test]
    fn get_unspent_and_find_unspent_work() {
        let (miner, _) = keypair();
        let c = chain_with_coinbase(&miner);
        let (utxos, next_id) = c.get_unspent_transactions();
        assert_eq!(utxos.len(), 2); /* coinbase and fee */
        assert_eq!(utxos[0].amount, 50);
        assert_eq!(next_id, 3); /* coinbase -> fee ->  */
        assert!(c.find_unspent_transaction(1).is_some());
        assert!(c.find_unspent_transaction(999).is_none());
    }

    #[test]
    fn get_balance_sums_unspent_by_address() {
        let (a, _) = keypair();
        let (b, _) = keypair();

        let g = genesis_block();
        let b1 = dummy_block(&g, vec![coinbase_transaction(&a, 0)], 1);
        let b2 = dummy_block(&b1, vec![coinbase_transaction(&b, 1)], 2);
        let c = crate::blockchain::chain::Chain {
            blocks: vec![g, b1, b2],
        };

        assert_eq!(c.get_balance(&a), 50);
        assert_eq!(c.get_balance(&b), 50);
    }
}
