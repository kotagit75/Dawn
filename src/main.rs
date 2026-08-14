#[macro_use]
extern crate log;
extern crate simple_logger as logger;

extern crate regex;

use log::Level;
use std::sync::Arc;

use crate::{
    beacon::InMemoryBeaconCache, chain_repository::FileChainRepository, config::CONFIG,
    key_repository::FileKeyRepository, node::Node,
};

pub mod api;
pub mod beacon;
pub mod blockchain;
pub mod chain_repository;
pub mod config;
pub mod event;
pub mod key_repository;
pub mod node;
pub mod p2p;
pub mod state;
pub mod util;

#[tokio::main]
async fn main() {
    logger::init_with_level(Level::Info).unwrap();

    let Some(node) = Node::new(
        Box::new(FileChainRepository::new("chain")),
        Box::new(FileKeyRepository::new("key.der")),
        Arc::new(InMemoryBeaconCache::new()),
    )
    .await
    else {
        error!("failed to initialize the node.");
        return;
    };
    node.run().await;
}
