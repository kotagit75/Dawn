use std::{
    io::Error,
    net::{Ipv4Addr, SocketAddr},
    str::FromStr,
};

use ::futures::future::join_all;
use axum::{Router, extract, response, routing::post};
use reqwest::Response;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;

use crate::{
    CONFIG,
    blockchain::{block::Block, transaction::Transaction},
    update::{Command, event::Event},
};

pub async fn init_p2p(event_tx: mpsc::Sender<Command>) -> Result<(), Error> {
    let app = Router::new()
        .route("/", post(handle_post_message))
        .with_state(event_tx);
    let addr = SocketAddr::new(
        std::net::IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
        CONFIG.internal_config.p2p_port,
    );
    let listener = tokio::net::TcpListener::bind(addr).await?;
    info!("P2P server is running on http://{}/", addr);
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum P2PMessage {
    QueryLatest,
    QueryAll,
    QueryTransactions,
    QueryPeers,
    ResponseBlockChain(Vec<Block>),
    ResponseTransactions(Vec<Transaction>),
    ResponsePeers(Vec<Peer>),
}

async fn handle_post_message(
    extract::State(event_tx): extract::State<mpsc::Sender<Command>>,
    extract::ConnectInfo(peer_addr): extract::ConnectInfo<SocketAddr>,
    extract::Json(message): extract::Json<P2PMessage>,
) -> response::Json<bool> {
    response::Json(
        event_tx
            .send(Command::Event(Event::P2PMessage(
                Ipv4Addr::from_str(&peer_addr.ip().to_string())
                    .map(|ip| Peer::new(ip))
                    .ok(),
                message,
            )))
            .await
            .is_ok(),
    )
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Peer {
    pub ip: Ipv4Addr,
}
impl Peer {
    pub fn new(ip: Ipv4Addr) -> Self {
        Self { ip }
    }
    pub fn get_url(&self) -> String {
        format!(
            "http://{}/",
            SocketAddr::new(
                std::net::IpAddr::V4(self.ip),
                CONFIG.internal_config.p2p_port
            )
        )
    }
    pub async fn write(&self, message: &P2PMessage) -> Result<Response, reqwest::Error> {
        reqwest::Client::new()
            .post(self.get_url())
            .json(message)
            .send()
            .await
            .inspect_err(|err| error!("failed to send message to peer({}): {:?}", self.ip, err))
    }
}

pub async fn broadcast(peers: &[Peer], message: &P2PMessage) -> Vec<Peer> {
    let tasks = peers.iter().map(|peer| {
        let peer_clone = peer.clone();
        let message_clone = message.clone();
        tokio::spawn(async move {
            match peer_clone.write(&message_clone).await {
                Ok(_) => None,
                Err(_) => Some(peer_clone),
            }
        })
    });
    join_all(tasks)
        .await
        .into_iter()
        .flatten()
        .flatten()
        .collect()
}
