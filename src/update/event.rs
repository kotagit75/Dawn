use serde::{Deserialize, Serialize};

use crate::{
    blockchain::{address::Address, block::Block},
    p2p::{P2PMessage, Peer},
};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum Event {
    AddPeer(Peer),
    RemovePeers(Vec<Peer>),
    AddTransaction(Address, u64, u64),
    MineBlock,
    CompletedMineBlock(Block),
    P2PMessage(Option<Peer>, P2PMessage),
}
