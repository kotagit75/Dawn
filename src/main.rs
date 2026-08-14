#[macro_use]
extern crate log;
extern crate simple_logger as logger;

extern crate regex;

use log::Level;
use std::sync::Arc;
use tokio::sync::{mpsc, watch};

use crate::{
    beacon::InMemoryBeaconCache,
    chain_repository::{ChainRepository, FileChainRepository},
    config::CONFIG,
    event::{Event, command::Command},
    key_repository::{FileKeyRepository, KeyRepository},
    p2p::Peer,
    state::State,
};

pub mod api;
pub mod beacon;
pub mod blockchain;
pub mod chain_repository;
pub mod config;
pub mod event;
pub mod key_repository;
pub mod p2p;
pub mod state;
pub mod util;

#[tokio::main]
async fn main() {
    logger::init_with_level(Level::Info).unwrap();

    let chain_repo = FileChainRepository::new("chain");
    let key_repo = FileKeyRepository::new("key.der");

    let Some(mut state) = init_state(&chain_repo, &key_repo) else {
        return;
    };

    let (event_tx, mut event_rx) = mpsc::channel(256);
    let (state_tx, state_rx) = watch::channel(state.clone());
    init_p2p_and_api(state_rx, event_tx.clone()).await;
    let beacon_cache = Arc::new(InMemoryBeaconCache::new());

    if CONFIG.args.mining {
        let _ = event_tx.send(Command::Event(Event::MineBlock)).await;
    }
    if let Some(address) = CONFIG.args.peer.clone() {
        let _ = event_tx
            .send(Command::Event(Event::AddPeer(Peer::new(&address))))
            .await;
    }

    while let Some(command) = event_rx.recv().await {
        let event = command.into_event();

        let result = event.process(&mut state, beacon_cache.as_ref()).await;
        if result.chain_changed {
            let _ = chain_repo
                .save(&state.chain)
                .inspect_err(|e| error!("failed to save chain: {}", e));
            let _ = state_tx.send(state.clone());
        }

        if let Some(response_tx) = command.into_response_tx() {
            let _ = response_tx.send(result.clone());
        }

        let event_tx_clone = event_tx.clone();
        let state_clone = state.clone();
        tokio::spawn(async move {
            let events = result.effect.run(state_clone).await;
            for event in events {
                let _ = event_tx_clone.send(Command::Event(event)).await;
            }
        });
    }
}

fn init_state(chain_repo: &dyn ChainRepository, key_repo: &dyn KeyRepository) -> Option<State> {
    debug!("loading node key");
    let Ok(sk) = key_repo.load_or_init() else {
        error!("failed to load node key");
        return None;
    };
    debug!("loading chain");
    let Ok(chain) = chain_repo.load_or_init() else {
        error!("failed to load chain");
        return None;
    };
    debug!("initializing state");
    Some(state::State::new(sk, chain))
}

async fn init_p2p_and_api(state_rx: watch::Receiver<State>, event_tx: mpsc::Sender<Command>) -> () {
    let event_tx_clone = event_tx.clone();
    tokio::spawn(async move {
        api::init_api(event_tx_clone, state_rx)
            .await
            .expect_err("failed to init api");
    });
    tokio::spawn(async move {
        p2p::init_p2p(event_tx)
            .await
            .expect_err("failed to init p2p");
    });
}
