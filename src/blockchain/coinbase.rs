use crate::{
    blockchain::{address::Address, transaction::Transaction, utxo::TransactionOut},
    util::signature::SignatureWrapper,
};

/*
 * COINBASE_AMOUNT_HALVING_INTERVAL and INITIAL_COINBASE_AMOUNT are subject to change in the future.
 */
const COINBASE_AMOUNT_HALVING_INTERVAL: u64 = 210000;
const INITIAL_COINBASE_AMOUNT: u64 = 50;
pub fn coinbase_amount(block_height: u64) -> u64 {
    let halvings: u64 = block_height / COINBASE_AMOUNT_HALVING_INTERVAL;
    INITIAL_COINBASE_AMOUNT >> halvings
}

pub fn coinbase_address() -> Address {
    Address { der: String::new() }
}
pub fn coinbase_transaction(address: &Address, block_height: u64) -> Transaction {
    Transaction {
        sender: coinbase_address(),
        out: vec![TransactionOut {
            address: address.clone(),
            amount: coinbase_amount(block_height),
        }],
        tx_in: Vec::new(),
        fee: 0,
        signature: SignatureWrapper::default(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        blockchain::validation::is_valid_coinbase_transaction,
        util::key::{SK, generate_sk},
    };

    fn keypair() -> (Address, SK) {
        let sk = generate_sk(512);
        let pk = sk.to_pk();
        (pk, sk)
    }
    #[test]
    fn coinbase_transaction_is_valid() {
        let (miner, _) = keypair();
        let block_height = 1;
        let tx = coinbase_transaction(&miner, block_height);
        assert!(is_valid_coinbase_transaction(&tx, block_height));
    }

    #[test]
    fn coinbase_transaction_invalid_when_amount_changed() {
        let (miner, _) = keypair();
        let block_height = 1;
        let mut tx = coinbase_transaction(&miner, block_height);
        tx.out[0].amount = 999;
        assert!(!is_valid_coinbase_transaction(&tx, block_height));
    }
}
