use std::net::{Ipv4Addr, SocketAddr};

use axum::{Router, extract, response, routing::post};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;

use crate::{
    blockchain::{block::Block, transaction::Transaction},
    update::Event,
};

const P2P_PORT: u16 = 62697;
pub async fn init_p2p(event_tx: mpsc::Sender<Event>) {
    let app = Router::new()
        .route("/", post(handle_post_message))
        .with_state(event_tx);
    let listener = tokio::net::TcpListener::bind(SocketAddr::new(
        std::net::IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
        P2P_PORT,
    ))
    .await
    .unwrap();
    println!("P2P server is running on http://localhost:{}", P2P_PORT);
    axum::serve(listener, app).await.unwrap();
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum P2PMessage {
    QueryLatest,
    QueryAll,
    QueryTransactions,
    ResponseBlockChain(Vec<Block>),
    ResponseTransactions(Vec<Transaction>),
}

async fn handle_post_message(
    extract::State(event_tx): extract::State<mpsc::Sender<Event>>,
    extract::Json(message): extract::Json<P2PMessage>,
) -> response::Json<bool> {
    response::Json(event_tx.send(Event::P2PMessage(message)).await.is_ok())
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Peer {
    pub ip: String,
}
impl Peer {
    pub fn new(ip: String) -> Self {
        Self { ip }
    }
    pub fn get_url(&self) -> String {
        format!("http://{}:{}/", self.ip, P2P_PORT)
    }
    pub async fn write(&self, message: &P2PMessage) {
        let _ = reqwest::Client::new()
            .post(&self.get_url())
            .json(message)
            .send()
            .await;
    }
}

pub async fn broadcast(peers: &[Peer], message: &P2PMessage) {
    for peer in peers {
        peer.write(message).await;
    }
}
