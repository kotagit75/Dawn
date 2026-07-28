use serde::{Deserialize, Serialize};

use crate::{blockchain::transaction::Transaction, p2p::P2PMessage};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum Effect {
    None,
    MineBlock(Vec<Transaction>),
    Broadcast(P2PMessage),
}

pub fn when_changed(effect: Effect, changed: bool) -> Effect {
    if changed { effect } else { Effect::None }
}
