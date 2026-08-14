#[macro_use]
extern crate log;
extern crate simple_logger as logger;

extern crate regex;

use clap::Parser;
use log::Level;
use std::sync::Arc;

use crate::{
    beacon::InMemoryBeaconCache, chain_repository::FileChainRepository, config::Config,
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

pub const VDF_DIFFICULTY: u64 = 5295676;

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Whether to mine blocks
    #[arg(short, long)]
    pub mining: bool,

    /// The IP address to add to the peer list
    #[arg(short, long)]
    pub peer: Option<String>,

    /// The port to listen on for the API
    #[arg(short, long, default_value = "8080")]
    pub api_port: u16,

    /// The port to listen on for the P2P network
    #[arg(long, default_value = "62697")]
    pub p2p_port: u16,

    /// The timeout for API requests in seconds
    #[arg(short, long, default_value = "5")]
    pub beacon_timeout: u64,

    /// Beacon provider command to run over stdio
    #[arg(long = "beacon-cmd", num_args = 1.., value_name = "CMD")]
    pub beacon_cmd: Vec<String>,

    /// For testing only: vdf difficulty
    #[arg(long)]
    pub vdf_difficulty: Option<u64>,
}

#[tokio::main]
async fn main() {
    logger::init_with_level(Level::Info).unwrap();

    let config = {
        let args = Args::parse();
        Config {
            mining: args.mining,
            peer: args.peer,
            api_port: args.api_port,
            p2p_port: args.p2p_port,
            beacon_timeout: args.beacon_timeout,
            beacon_cmd: args.beacon_cmd,
            vdf_difficulty: args.vdf_difficulty.unwrap_or(VDF_DIFFICULTY),
        }
    };

    let Ok(node) = Node::new(
        config,
        Box::new(FileChainRepository::new("chain")),
        Box::new(FileKeyRepository::new("key.der")),
        Arc::new(InMemoryBeaconCache::new()),
    )
    .await
    else {
        return;
    };
    node.run().await;
}
