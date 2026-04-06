use openssl::error::ErrorStack;
use serde::{Deserialize, Serialize};

use crate::{
    blockchain::address::Address,
    util::{key::SK, signature::Signature},
};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Transaction {
    pub sender: Address,
    pub recipient: Address,
    pub amount: u64,
    pub signature: Signature,
}

impl std::fmt::Display for Transaction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} -> {}: {}", self.sender, self.recipient, self.amount)?;
        Ok(())
    }
}

impl Transaction {
    pub fn new(sender: Address, recipient: Address, amount: u64, signature: Signature) -> Self {
        Self {
            sender,
            recipient,
            amount,
            signature,
        }
    }
    pub fn new_with_creating_signature(
        sender: &Address,
        recipient: &Address,
        amount: u64,
        sk: &SK,
    ) -> Result<Self, ErrorStack> {
        Ok(Self {
            sender: sender.clone(),
            recipient: recipient.clone(),
            amount,
            signature: create_transaction_signature(sender, recipient, amount, sk)?,
        })
    }
    pub fn verify_signature(&self) -> bool {
        self.sender.clone().verify(
            transacction_to_buf_for_signature(&self.sender, &self.recipient, self.amount)
                .as_slice(),
            &self.signature,
        )
    }

    pub fn is_valid(&self) -> bool {
        self.verify_signature() && self.amount > 0
    }
}

fn transacction_to_buf_for_signature(
    sender: &Address,
    recipient: &Address,
    amount: u64,
) -> Vec<u8> {
    format!("{sender}{recipient}{amount}").as_bytes().to_vec()
}

fn create_transaction_signature(
    sender: &Address,
    recipient: &Address,
    amount: u64,
    sk: &SK,
) -> Result<Signature, ErrorStack> {
    let data = transacction_to_buf_for_signature(sender, recipient, amount);
    sk.sign(&data)
}
