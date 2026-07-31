use std::{
    io::Error,
    net::{Ipv4Addr, SocketAddr},
};

use ::futures::future::join_all;
use serde::{Deserialize, Serialize};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    sync::mpsc,
};

use crate::{
    CONFIG,
    blockchain::{block::Block, transaction::Transaction},
    update::{Command, event::Event},
};

pub async fn init_p2p(event_tx: mpsc::Sender<Command>) -> Result<(), Error> {
    let addr = SocketAddr::new(
        std::net::IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
        CONFIG.internal_config.p2p_port,
    );
    let listener = tokio::net::TcpListener::bind(addr).await?;
    info!("P2P server is running on {}", addr);
    loop {
        let (mut socket, mut peer_addr) = listener.accept().await?;
        let mut buf = String::new();
        socket.read_to_string(&mut buf).await?;
        if let Ok(message) = serde_json::from_str(&buf) {
            peer_addr.set_port(CONFIG.internal_config.p2p_port);
            handle_post_message(event_tx.clone(), peer_addr, message).await;
        }
    }
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
    event_tx: mpsc::Sender<Command>,
    peer_addr: SocketAddr,
    message: P2PMessage,
) -> bool {
    event_tx
        .send(Command::Event(Event::P2PMessage(
            Some(Peer::new(peer_addr.to_string())),
            message,
        )))
        .await
        .is_ok()
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Peer {
    pub addr: String,
}
impl Peer {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }
    pub async fn write(&self, message: &P2PMessage) -> Result<(), Error> {
        let mut stream = tokio::net::TcpStream::connect(&self.addr).await?;
        stream
            .write_all(&serde_json::to_vec(message)?)
            .await
            .inspect_err(|err| error!("failed to send message to peer({}): {:?}", self.addr, err))
            .ok();
        Ok(())
    }
}

pub async fn broadcast(peers: &[Peer], message: &P2PMessage) -> Vec<Peer> {
    let tasks = peers.iter().map(|peer| {
        let peer_clone = peer.clone();
        let message_clone = message.clone();
        tokio::spawn(async move {
            peer_clone
                .write(&message_clone)
                .await
                .ok()
                .map_or(Some(peer_clone), |_| None)
        })
    });
    join_all(tasks)
        .await
        .into_iter()
        .flatten()
        .flatten()
        .collect()
}
