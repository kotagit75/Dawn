use crate::{
    address::Address,
    block::{Block, BlockData, BlockDataOwned},
    chain::Chain,
    coinbase::coinbase_transaction,
    transaction::Transaction,
    utxo::{flex_unspent_transactions, get_transaction_out, transactions_to_unspent_ids},
};
use btfy_beacon::Beacon;
use btfy_util::key::SK;

impl Chain {
    pub fn generate_next_block_data(
        &self,
        issuer: &Address,
        beacon: Beacon,
        transactions_without_coinbase: Vec<Transaction>,
        next_timestamp: i64,
    ) -> BlockDataOwned {
        let previous_block: Block = self.get_latest_block();
        let next_index: u64 = previous_block.index + 1;
        let transactions = [coinbase_transaction(issuer, next_index)]
            .iter()
            .chain(&transactions_without_coinbase)
            .cloned()
            .collect::<Vec<Transaction>>();
        BlockData::new(
            next_index,
            next_timestamp,
            &transactions,
            &beacon,
            issuer,
            previous_block.hash,
        )
        .to_owned()
    }

    pub fn generate_next_block(
        &self,
        sk: &SK,
        vdf_solution: Vec<u8>,
        block_data: BlockDataOwned,
    ) -> Block {
        Block::new_with_creating_signature(&block_data.as_borrowed(), vdf_solution, sk)
    }

    pub fn generate_transaction(
        &self,
        sender: &Address,
        recipient: &Address,
        send_amount: u64,
        secret_key: &SK,
        used_transactions: &[Transaction],
        fee: u64,
    ) -> Option<Transaction> {
        let amount = send_amount + fee;

        let mut filtered_unspent_transactions = self.filter_unspent_transactions_by_address(sender);
        let used_unspent_ids: Vec<u64> = transactions_to_unspent_ids(used_transactions);
        filtered_unspent_transactions.retain(|tx| !used_unspent_ids.contains(&tx.id));
        let use_unspent = flex_unspent_transactions(amount, filtered_unspent_transactions);
        if use_unspent.is_empty() {
            return None;
        }

        let transaction = Transaction::new_with_creating_signature(
            sender,
            get_transaction_out(
                sender,
                recipient,
                send_amount,
                fee,
                use_unspent.iter().map(|tx| tx.amount).sum::<u64>(),
            ),
            use_unspent.iter().map(|tx| tx.to_txin()).collect(),
            fee,
            secret_key,
        );
        Some(transaction)
    }
}

#[cfg(test)]
mod tests {
    use crate::address::Address;
    use crate::block::{Block, genesis_block};
    use crate::utxo::TransactionIn;
    use crate::{chain::Chain, coinbase::coinbase_transaction, transaction::Transaction};
    use btfy_beacon::Beacon;
    use btfy_util::key::{SK, generate_sk};
    use btfy_util::signature::SignatureWrapper;

    fn keypair() -> (Address, SK) {
        let sk = generate_sk(512);
        let pk = sk.to_pk();
        (pk, sk)
    }

    fn dummy_block(prev: &Block, txs: Vec<Transaction>, beacon: i32) -> Block {
        Block {
            index: prev.index + 1,
            timestamp: prev.timestamp + 1,
            transactions: txs,
            beacon: Beacon {
                values: vec![beacon],
            },
            vdf_solution: vec![],
            previous_hash: prev.hash,
            issuer: prev.issuer.clone(),
            signature: SignatureWrapper::default(),
            hash: [prev.index as u8 + 1; 32],
        }
    }

    fn chain_with_coinbase(miner: &Address) -> Chain {
        let g = genesis_block();
        let b1 = dummy_block(&g, vec![coinbase_transaction(miner, 1)], 1);
        Chain {
            blocks: vec![g, b1],
        }
    }

    #[test]
    fn generate_transaction_returns_none_when_insufficient() {
        let (sender, sk) = keypair();
        let (recipient, _) = keypair();
        let c = chain_with_coinbase(&sender);
        let tx = c.generate_transaction(&sender, &recipient, 999, &sk, &[], 0);
        assert!(tx.is_none());
    }

    #[test]
    fn generate_transaction_uses_utxo_and_returns_change() {
        let (sender, sk) = keypair();
        let (recipient, _) = keypair();
        let c = chain_with_coinbase(&sender);

        let tx = c
            .generate_transaction(&sender, &recipient, 30, &sk, &[], 0)
            .unwrap();

        assert_eq!(tx.tx_in, vec![TransactionIn { unspent_id: 1 }]);
        assert_eq!(tx.out.iter().map(|o| o.amount).sum::<u64>(), 50);
    }

    #[test]
    fn generate_transaction_respects_used_transactions_filter() {
        let (sender, sk) = keypair();
        let (recipient, _) = keypair();
        let c = chain_with_coinbase(&sender);

        let used = c
            .generate_transaction(&sender, &recipient, 30, &sk, &[], 0)
            .unwrap();

        let next = c.generate_transaction(&sender, &recipient, 10, &sk, &[used], 0);

        assert!(next.is_none());
    }

    #[test]
    fn generate_transaction_returns_none_when_amount_plus_fee_exceeds_funds() {
        let (sender, sk) = keypair();
        let (recipient, _) = keypair();
        let c = chain_with_coinbase(&sender);

        let tx = c.generate_transaction(&sender, &recipient, 49, &sk, &[], 2);

        assert!(tx.is_none());
    }
}
