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
