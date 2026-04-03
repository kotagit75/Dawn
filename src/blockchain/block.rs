use openssl::error::ErrorStack;
use serde::{Deserialize, Serialize};

use crate::{
    blockchain::{address::Address, beacon::Beacon, transaction::Transaction},
    util::{
        hash::{Hashed, hash},
        key::{PK, SK},
        signature::Signature,
    },
};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Block {
    pub index: u64,
    pub timestamp: u64,
    pub transactions: Vec<Transaction>,
    pub beacon: Beacon,
    pub previous_hash: Hashed,
    pub issuer: Address,
    pub signature: Signature,
    pub hash: Hashed,
}

impl Block {
    pub fn new(
        index: u64,
        timestamp: u64,
        transactions: Vec<Transaction>,
        beacon: Beacon,
        issuer: &Address,
        previous_hash: Hashed,
        signature: Signature,
    ) -> Self {
        let hash = calculate_hash(
            index,
            timestamp,
            &transactions,
            beacon.clone(),
            issuer,
            previous_hash,
            signature.clone(),
        );
        Self {
            index,
            timestamp,
            transactions,
            beacon,
            previous_hash,
            issuer: issuer.clone(),
            signature,
            hash,
        }
    }
    pub fn new_with_creating_signature(
        index: u64,
        timestamp: u64,
        transactions: Vec<Transaction>,
        beacon: Beacon,
        issuer: &Address,
        previous_hash: Hashed,
        sk: &SK,
    ) -> Result<Self, ErrorStack> {
        Ok(Self::new(
            index,
            timestamp,
            transactions.clone(),
            beacon.clone(),
            issuer,
            previous_hash,
            create_block_signature(
                index,
                timestamp,
                &transactions,
                beacon.clone(),
                issuer,
                previous_hash,
                sk,
            )?,
        ))
    }
    pub fn verify(&self) -> bool {
        self.issuer.verify(
            block_to_buf_for_signature(
                self.index,
                self.timestamp,
                &self.transactions,
                self.beacon.clone(),
                &self.issuer,
                self.previous_hash.clone(),
            )
            .as_slice(),
            &self.signature,
        )
    }
}

pub fn calculate_hash(
    index: u64,
    timestamp: u64,
    transactions: &[Transaction],
    beacon: Beacon,
    issuer: &Address,
    previous_hash: Hashed,
    signature: Signature,
) -> Hashed {
    hash(
        format!(
            "{index}{timestamp}{transactions:?}{beacon:?}{issuer:?}{previous_hash:?}{signature:?}"
        )
        .as_bytes(),
    )
}

fn block_to_buf_for_signature(
    index: u64,
    timestamp: u64,
    transactions: &[Transaction],
    beacon: Beacon,
    issuer: &Address,
    previous_hash: Hashed,
) -> Vec<u8> {
    format!("{index}{timestamp}{transactions:?}{beacon:?}{previous_hash:?}{issuer:?}")
        .as_bytes()
        .to_vec()
}

fn create_block_signature(
    index: u64,
    timestamp: u64,
    transactions: &[Transaction],
    beacon: Beacon,
    issuer: &Address,
    previous_hash: Hashed,
    sk: &SK,
) -> Result<Signature, ErrorStack> {
    let data = block_to_buf_for_signature(
        index,
        timestamp,
        transactions,
        beacon,
        issuer,
        previous_hash,
    );
    sk.sign(&data)
}

pub fn genesis_block() -> Block {
    let pk = PK {
        der: "".to_string(),
    };
    Block::new(
        0,
        0,
        Vec::new(),
        Beacon { value: 0 },
        &pk,
        [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0,
        ],
        Vec::new(),
    )
}
