use std::sync::Arc;

use tokio::sync::{mpsc, watch};

use crate::{
    api,
    beacon::BeaconCache,
    chain_repository::ChainRepository,
    config::CONFIG,
    event::{Event, command::Command},
    key_repository::KeyRepository,
    p2p::{self, Peer},
    state::State,
};

pub struct Node {
    state: State,
    chain_repo: Box<dyn ChainRepository>,
    event_tx: mpsc::Sender<Command>,
    event_rx: mpsc::Receiver<Command>,
    state_tx: watch::Sender<State>,
    beacon_cache: Arc<dyn BeaconCache>,
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
    Some(State::new(sk, chain))
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

impl Node {
    pub async fn new(
        chain_repo: Box<dyn ChainRepository>,
        key_repo: Box<dyn KeyRepository>,
        beacon_cache: Arc<dyn BeaconCache>,
    ) -> Option<Self> {
        let state = init_state(chain_repo.as_ref(), key_repo.as_ref())?;
        let (event_tx, event_rx) = mpsc::channel(256);
        let (state_tx, state_rx) = watch::channel(state.clone());
        init_p2p_and_api(state_rx, event_tx.clone()).await;
        Some(Self {
            chain_repo,
            beacon_cache,
            state,
            event_tx,
            event_rx,
            state_tx,
        })
    }

    pub async fn run(mut self) {
        self.dispatch_initial_events().await;
        self.run_event_loop().await;
    }

    async fn dispatch_initial_events(&mut self) {
        if CONFIG.args.mining {
            let _ = self.event_tx.send(Command::Event(Event::MineBlock)).await;
        }
        if let Some(address) = CONFIG.args.peer.clone() {
            let _ = self
                .event_tx
                .send(Command::Event(Event::AddPeer(Peer::new(&address))))
                .await;
        }
    }

    async fn run_event_loop(&mut self) {
        while let Some(command) = self.event_rx.recv().await {
            let event = command.into_event();
            let result = event
                .process(&mut self.state, self.beacon_cache.as_ref())
                .await;
            if result.chain_changed {
                let _ = self
                    .chain_repo
                    .save(&self.state.chain)
                    .inspect_err(|e| error!("failed to save chain: {}", e));
            }
            if result.changed {
                let _ = self.state_tx.send(self.state.clone());
            }
            if let Some(response_tx) = command.into_response_tx() {
                let _ = response_tx.send(result.clone());
            }
            let event_tx_clone = self.event_tx.clone();
            let state_clone = self.state.clone();
            tokio::spawn(async move {
                let events = result.effect.run(state_clone).await;
                for event in events {
                    let _ = event_tx_clone.send(Command::Event(event)).await;
                }
            });
        }
    }
}
