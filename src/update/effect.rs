use serde::{Deserialize, Serialize};

use crate::{blockchain::transaction::Transaction, p2p::P2PMessage};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum Effect {
    None,
    MineBlock(Vec<Transaction>),
    Broadcast(P2PMessage),
}

pub fn map_effect(effect: impl FnOnce() -> Effect, changed: bool) -> Effect {
    if changed { effect() } else { Effect::None }
}
