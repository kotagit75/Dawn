use bitcode::{Decode, Encode};
use btfy_beacon::Beacon;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Debug, Display};
use vdf_rs::InvalidIterations;

use crate::{address::Address, transaction::Transaction};
use btfy_util::{
    hash::{Hashed, hash},
    key::{PK, SK},
    signature::SignatureWrapper,
    vdf::{solution_to_string, solve, verify_solution},
};

pub const MAX_TRANSACTIONS_PER_BLOCK: usize = 100;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Encode, Decode)]
pub struct Block {
    pub index: u64,
    pub timestamp: i64,
    pub transactions: Vec<Transaction>,
    pub beacon: Beacon,
    pub vdf_solution: Vec<u8>,
    pub previous_hash: Hashed,
    pub issuer: Address,
    pub signature: SignatureWrapper,
    pub hash: Hashed,
}

impl Block {
    pub fn new(blockdata: &BlockData, vdf_solution: Vec<u8>, signature: SignatureWrapper) -> Self {
        let hash = calculate_hash(blockdata, &vdf_solution, signature.clone());
        Self {
            index: blockdata.index,
            timestamp: blockdata.timestamp,
            transactions: blockdata.transactions.to_vec(),
            beacon: blockdata.beacon.clone(),
            vdf_solution,
            previous_hash: blockdata.previous_hash,
            issuer: blockdata.issuer.clone(),
            signature,
            hash,
        }
    }
    pub fn new_with_creating_signature(
        blockdata: &BlockData,
        vdf_solution: Vec<u8>,
        sk: &SK,
    ) -> Self {
        Self::new(
            blockdata,
            vdf_solution.clone(),
            create_block_signature(blockdata, &vdf_solution, sk),
        )
    }
    pub fn verify_signature(&self) -> bool {
        self.issuer.verify(
            block_to_buf_for_signature(&self.to_blockdata(), &self.vdf_solution).as_slice(),
            &self.signature,
        )
    }
    pub fn verify_vdf_solution(&self, difficulty: u64) -> bool {
        verify_solution(
            difficulty,
            block_to_buf_for_vdf(&self.to_blockdata()).as_slice(),
            &self.vdf_solution,
        )
    }

    pub fn get_block_height(&self) -> u64 {
        self.index
    }

    fn to_blockdata(&self) -> BlockData<'_> {
        BlockData::new(
            self.index,
            self.timestamp,
            &self.transactions,
            &self.beacon,
            &self.issuer,
            self.previous_hash,
        )
    }

    pub fn calculate_hash(&self) -> Hashed {
        calculate_hash(
            &self.to_blockdata(),
            &self.vdf_solution,
            self.signature.clone(),
        )
    }
}

pub struct BlockData<'a> {
    index: u64,
    timestamp: i64,
    transactions: &'a [Transaction],
    beacon: &'a Beacon,
    issuer: &'a Address,
    previous_hash: Hashed,
}
impl<'a> BlockData<'a> {
    pub fn new(
        index: u64,
        timestamp: i64,
        transactions: &'a [Transaction],
        beacon: &'a Beacon,
        issuer: &'a Address,
        previous_hash: Hashed,
    ) -> Self {
        Self {
            index,
            timestamp,
            transactions,
            beacon,
            issuer,
            previous_hash,
        }
    }
}
impl<'a> Display for BlockData<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}{:?}{:?}{:?}{:?}",
            self.index,
            self.timestamp,
            self.transactions,
            self.beacon,
            self.issuer,
            self.previous_hash
        )
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct BlockDataOwned {
    pub index: u64,
    pub timestamp: i64,
    pub transactions: Vec<Transaction>,
    pub beacon: Beacon,
    pub issuer: Address,
    pub previous_hash: Hashed,
}

impl std::fmt::Display for BlockDataOwned {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}{:?}{:?}{:?}{:?}",
            self.index,
            self.timestamp,
            self.transactions,
            self.beacon,
            self.issuer,
            self.previous_hash
        )
    }
}

impl<'a> BlockData<'a> {
    pub fn to_owned(&self) -> BlockDataOwned {
        BlockDataOwned {
            index: self.index,
            timestamp: self.timestamp,
            transactions: self.transactions.to_vec(),
            beacon: self.beacon.clone(),
            issuer: self.issuer.clone(),
            previous_hash: self.previous_hash,
        }
    }
}
impl BlockDataOwned {
    pub fn as_borrowed(&self) -> BlockData<'_> {
        BlockData::new(
            self.index,
            self.timestamp,
            &self.transactions,
            &self.beacon,
            &self.issuer,
            self.previous_hash,
        )
    }
}

pub fn calculate_hash(
    blockdata: &BlockData,
    vdf_solution: &[u8],
    signature: SignatureWrapper,
) -> Hashed {
    hash(
        format!(
            "{}{}{:?}",
            blockdata,
            solution_to_string(vdf_solution),
            signature
        )
        .as_bytes(),
    )
}

fn block_to_buf_for_signature(blockdata: &BlockData, vdf_solution: &[u8]) -> Vec<u8> {
    format!("{}{}", blockdata, solution_to_string(vdf_solution))
        .as_bytes()
        .to_vec()
}

fn create_block_signature(blockdata: &BlockData, vdf_solution: &[u8], sk: &SK) -> SignatureWrapper {
    let data = block_to_buf_for_signature(blockdata, vdf_solution);
    sk.sign(&data)
}

const GENESIS_BLOCK_DATA: &str = include_str!("genesis.txt");
pub fn genesis_block() -> Block {
    let pk = PK {
        der: GENESIS_BLOCK_DATA.to_string(),
    };
    let blockdata = BlockData {
        index: 0,
        timestamp: 0,
        transactions: &[],
        beacon: &Beacon { values: Vec::new() },
        previous_hash: [0; 32],
        issuer: &pk,
    };
    Block::new(&blockdata, Vec::new(), SignatureWrapper::default())
}

fn block_to_buf_for_vdf(blockdata: &BlockData) -> Vec<u8> {
    blockdata.to_string().as_bytes().to_vec()
}
pub fn solve_block_vdf(
    blockdata: &BlockDataOwned,
    difficulty: u64,
) -> Result<Vec<u8>, InvalidIterations> {
    solve(
        block_to_buf_for_vdf(&blockdata.as_borrowed()).as_slice(),
        difficulty,
    )
}

#[cfg(test)]
mod tests {
    use crate::{
        coinbase::coinbase_transaction,
        utxo::{TransactionIn, TransactionOut, UnspentTransaction},
    };
    use btfy_util::key::generate_sk;

    use super::*;

    fn keypair() -> (Address, SK) {
        let sk = generate_sk(512);
        let pk = sk.to_pk();
        (pk, sk)
    }

    const TEST_VDF_DIFFICULTY: u64 = 10;

    #[test]
    fn get_unspent_transactions_adds_fee_to_miner_utxo() {
        let (sender, sk) = keypair();
        let (recipient, _) = keypair();
        let (miner, _) = keypair();

        let tx = Transaction::new_with_creating_signature(
            &sender,
            vec![TransactionOut {
                address: recipient,
                amount: 10,
            }],
            vec![TransactionIn { unspent_id: 1 }],
            2,
            &sk,
        );

        let prev = vec![UnspentTransaction {
            id: 1,
            address: sender,
            amount: 12,
        }];

        let b = Block {
            index: 1,
            timestamp: 1,
            transactions: vec![coinbase_transaction(&miner, 1), tx],
            beacon: Beacon { values: vec![] },
            vdf_solution: vec![],
            previous_hash: [0; 32],
            issuer: miner.clone(),
            signature: SignatureWrapper::default(),
            hash: [1; 32],
        };

        let (next, _) = b.get_unspent_transactions((prev, 2));

        assert!(next.iter().any(|u| u.address == miner && u.amount == 2));
    }

    #[test]
    fn is_invalid_when_too_many_transactions() {
        let (miner, _) = keypair();
        let transactions = vec![coinbase_transaction(&miner, 1); MAX_TRANSACTIONS_PER_BLOCK + 1];

        let block = Block {
            index: 1,
            timestamp: 1,
            transactions,
            beacon: Beacon { values: vec![] },
            vdf_solution: vec![],
            previous_hash: [0; 32],
            issuer: miner,
            signature: SignatureWrapper::default(),
            hash: [1; 32],
        };

        assert!(!block.is_valid(&[], TEST_VDF_DIFFICULTY));
    }
}
