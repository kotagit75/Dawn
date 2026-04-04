use openssl::error::ErrorStack;
use serde::{Deserialize, Serialize};
use vdf::InvalidIterations;

use crate::{
    beacon::Beacon,
    blockchain::{address::Address, transaction::Transaction},
    util::{
        hash::{Hashed, hash},
        key::{PK, SK},
        signature::Signature,
        vdf::{solve, verify_solution},
    },
};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Block {
    pub index: u64,
    pub timestamp: i64,
    pub transactions: Vec<Transaction>,
    pub beacon: Beacon,
    pub vdf_solution: Vec<u8>,
    pub previous_hash: Hashed,
    pub issuer: Address,
    pub signature: Signature,
    pub hash: Hashed,
}

impl Block {
    pub fn new(
        index: u64,
        timestamp: i64,
        transactions: Vec<Transaction>,
        beacon: Beacon,
        vdf_solution: Vec<u8>,
        issuer: &Address,
        previous_hash: Hashed,
        signature: Signature,
    ) -> Self {
        let hash = calculate_hash(
            index,
            timestamp,
            &transactions,
            beacon.clone(),
            &vdf_solution,
            issuer,
            previous_hash,
            signature.clone(),
        );
        Self {
            index,
            timestamp,
            transactions,
            beacon,
            vdf_solution,
            previous_hash,
            issuer: issuer.clone(),
            signature,
            hash,
        }
    }
    pub fn new_with_creating_signature(
        index: u64,
        timestamp: i64,
        transactions: Vec<Transaction>,
        beacon: Beacon,
        vdf_solution: Vec<u8>,
        issuer: &Address,
        previous_hash: Hashed,
        sk: &SK,
    ) -> Result<Self, ErrorStack> {
        Ok(Self::new(
            index,
            timestamp,
            transactions.clone(),
            beacon.clone(),
            vdf_solution.clone(),
            issuer,
            previous_hash,
            create_block_signature(
                index,
                timestamp,
                &transactions,
                beacon.clone(),
                &vdf_solution,
                issuer,
                previous_hash,
                sk,
            )?,
        ))
    }
    pub fn verify_signature(&self) -> bool {
        self.issuer.verify(
            block_to_buf_for_signature(
                self.index,
                self.timestamp,
                &self.transactions,
                self.beacon.clone(),
                &self.vdf_solution,
                &self.issuer,
                self.previous_hash.clone(),
            )
            .as_slice(),
            &self.signature,
        )
    }
    pub fn verify_vdf_solution(&self) -> bool {
        verify_solution(
            block_to_buf_for_vdf(
                self.index,
                self.timestamp,
                &self.transactions,
                self.beacon.clone(),
                &self.issuer,
                self.previous_hash.clone(),
            )
            .as_slice(),
            &self.vdf_solution,
        )
    }

    pub fn is_valid(&self) -> bool {
        self.verify_signature()
            && self.verify_vdf_solution()
            && self.transactions.iter().all(|t| t.is_valid())
    }

    pub fn calculate_hash(&self) -> Hashed {
        calculate_hash(
            self.index,
            self.timestamp,
            &self.transactions,
            self.beacon.clone(),
            &self.vdf_solution,
            &self.issuer,
            self.previous_hash.clone(),
            self.signature.clone(),
        )
    }
}

pub fn calculate_hash(
    index: u64,
    timestamp: i64,
    transactions: &[Transaction],
    beacon: Beacon,
    vdf_solution: &[u8],
    issuer: &Address,
    previous_hash: Hashed,
    signature: Signature,
) -> Hashed {
    hash(
        format!(
            "{index}{timestamp}{transactions:?}{beacon:?}{vdf_solution:?}{issuer:?}{previous_hash:?}{signature:?}"
        )
        .as_bytes(),
    )
}

fn block_to_buf_for_signature(
    index: u64,
    timestamp: i64,
    transactions: &[Transaction],
    beacon: Beacon,
    vdf_solution: &[u8],
    issuer: &Address,
    previous_hash: Hashed,
) -> Vec<u8> {
    format!(
        "{index}{timestamp}{transactions:?}{beacon:?}{vdf_solution:?}{previous_hash:?}{issuer:?}"
    )
    .as_bytes()
    .to_vec()
}

fn create_block_signature(
    index: u64,
    timestamp: i64,
    transactions: &[Transaction],
    beacon: Beacon,
    vdf_solution: &[u8],
    issuer: &Address,
    previous_hash: Hashed,
    sk: &SK,
) -> Result<Signature, ErrorStack> {
    let data = block_to_buf_for_signature(
        index,
        timestamp,
        transactions,
        beacon,
        vdf_solution,
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
        Vec::new(),
        &pk,
        [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0,
        ],
        Vec::new(),
    )
}

fn block_to_buf_for_vdf(
    index: u64,
    timestamp: i64,
    transactions: &[Transaction],
    beacon: Beacon,
    issuer: &Address,
    previous_hash: Hashed,
) -> Vec<u8> {
    format!("{index}{timestamp}{transactions:?}{beacon:?}{previous_hash:?}{issuer:?}")
        .as_bytes()
        .to_vec()
}
pub fn solve_block_vdf(
    index: u64,
    timestamp: i64,
    transactions: &[Transaction],
    beacon: Beacon,
    issuer: &Address,
    previous_hash: Hashed,
) -> Result<Vec<u8>, InvalidIterations> {
    solve(
        block_to_buf_for_vdf(
            index,
            timestamp,
            transactions,
            beacon,
            issuer,
            previous_hash,
        )
        .as_slice(),
    )
}
