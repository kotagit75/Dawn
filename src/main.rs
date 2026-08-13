#[macro_use]
extern crate log;
extern crate simple_logger as logger;

extern crate regex;

use log::Level;
use std::sync::Arc;
use tokio::sync::{mpsc, watch};

use crate::{
    beacon::InMemoryBeaconCache,
    config::CONFIG,
    effect::run_effect,
    node::save_chain,
    p2p::Peer,
    state::State,
    update::{Command, UpdateResult, event::Event, update},
};

pub mod api;
pub mod beacon;
pub mod blockchain;
pub mod config;
pub mod effect;
pub mod node;
pub mod p2p;
pub mod state;
pub mod update;
pub mod util;

#[tokio::main]
async fn main() {
    logger::init_with_level(Level::Info).unwrap();

    let Some(mut state) = init_state() else {
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
        let (event, response_tx) = match command {
            Command::Event(event) => (event, None),
            Command::ApiRequest(event, response_tx) => (event, Some(response_tx)),
        };
        let previous_state = state.clone();
        let effect = update(event, &mut state, beacon_cache.as_ref()).await;
        if state.chain != previous_state.chain {
            let _ = save_chain(&state.chain).inspect_err(|e| error!("failed to save chain: {}", e));
        }
        let _ = state_tx.send(state.clone());
        if let Some(response_tx) = response_tx {
            let _ = response_tx.send(UpdateResult {
                changed: state != previous_state,
                effect: effect.clone(),
            });
        }
        let event_tx_clone = event_tx.clone();
        let state_clone = state.clone();
        tokio::spawn(async move {
            let events = run_effect(state_clone, effect).await;
            for event in events {
                let _ = event_tx_clone.send(Command::Event(event)).await;
            }
        });
    }
}

fn init_state() -> Option<State> {
    debug!("loading node key");
    let Ok(sk) = node::load_or_generate_key() else {
        error!("failed to load node key");
        return None;
    };
    debug!("loading chain");
    let Ok(chain) = node::load_or_generate_chain() else {
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
