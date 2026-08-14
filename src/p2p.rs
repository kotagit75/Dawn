use std::{
    io::Error,
    net::{Ipv4Addr, SocketAddr},
};

use ::futures::future::join_all;
use bitcode::{Decode, Encode};
use serde::{Deserialize, Serialize};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    sync::mpsc,
};

use crate::{
    blockchain::{block::Block, transaction::Transaction},
    config::CONFIG,
    event::{Command, Event},
};

pub async fn init_p2p(event_tx: mpsc::Sender<Command>) -> Result<(), Error> {
    let addr = SocketAddr::new(
        std::net::IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
        CONFIG.args.p2p_port,
    );
    let listener = tokio::net::TcpListener::bind(addr).await?;
    info!("P2P server is running on {}", addr);
    loop {
        let (mut socket, mut peer_addr) = listener.accept().await?;
        let mut buf: Vec<u8> = Vec::new();
        socket.read_to_end(&mut buf).await?;
        if let Ok(payload) = bitcode::decode::<P2PMessagePayload>(&buf) {
            peer_addr.set_port(payload.port);
            handle_post_message(event_tx.clone(), peer_addr, payload.message).await;
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Encode, Decode, PartialEq)]
pub enum P2PMessage {
    QueryLatest,
    QueryAll,
    QueryTransactions,
    QueryPeers,
    ResponseBlockChain(Vec<Block>),
    ResponseTransactions(Vec<Transaction>),
    ResponsePeers(Vec<Peer>),
}

#[derive(Clone, Debug, Serialize, Deserialize, Encode, Decode, PartialEq)]
pub struct P2PMessagePayload {
    pub message: P2PMessage,
    pub port: u16,
}

async fn handle_post_message(
    event_tx: mpsc::Sender<Command>,
    peer_addr: SocketAddr,
    message: P2PMessage,
) -> bool {
    event_tx
        .send(Command::Event(Event::P2PMessage(
            Some(Peer::new_addr(peer_addr)),
            message,
        )))
        .await
        .is_ok()
}

#[derive(Clone, Debug, Serialize, Deserialize, Encode, Decode, PartialEq, Eq)]
pub struct Peer {
    pub addr: String,
}
impl Peer {
    pub fn new(addr: &str) -> Self {
        Self {
            addr: addr.to_string(),
        }
    }
    pub fn new_addr(addr: SocketAddr) -> Self {
        Self {
            addr: addr.to_string(),
        }
    }
    pub async fn write(&self, message: &P2PMessage) -> Result<(), Error> {
        let payload = P2PMessagePayload {
            message: message.clone(),
            port: CONFIG.args.p2p_port,
        };
        let mut stream = tokio::net::TcpStream::connect(&self.addr).await?;
        stream
            .write_all(&bitcode::encode(&payload))
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
