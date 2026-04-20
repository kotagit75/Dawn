use std::net::{Ipv4Addr, SocketAddr};

use axum::{
    Router,
    extract::{self, Path},
    response,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use tokio::sync::{mpsc, watch};

use crate::{blockchain::address::Address, p2p::Peer, state::State, update::Event, util::key::PK};

const API_PORT: u16 = 8080;
pub async fn init_api(event_tx: mpsc::Sender<Event>, state_rx: watch::Receiver<State>) {
    let app = Router::new()
        .route("/state", get(handle_get_state))
        .route("/address", get(handle_get_address))
        .route("/balance", get(handle_get_balance))
        .route("/balance/{address}", get(handle_get_balance_with_address))
        .route("/tx", post(handle_post_transaction))
        .route("/mine", post(handle_post_mine))
        .route("/peer", post(handle_post_peer))
        .with_state((event_tx, state_rx));
    let listener = tokio::net::TcpListener::bind(SocketAddr::new(
        std::net::IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
        API_PORT,
    ))
    .await
    .unwrap();
    info!("API server is running on http://localhost:{}", API_PORT);
    axum::serve(listener, app).await.unwrap();
}

async fn handle_get_state(
    extract::State((_, state_rx)): extract::State<(mpsc::Sender<Event>, watch::Receiver<State>)>,
) -> response::Json<State> {
    response::Json(state_rx.borrow().clone())
}

async fn handle_get_address(
    extract::State((_, state_rx)): extract::State<(mpsc::Sender<Event>, watch::Receiver<State>)>,
) -> String {
    state_rx.borrow().clone().address.der
}

async fn handle_get_balance(
    extract::State((_, state_rx)): extract::State<(mpsc::Sender<Event>, watch::Receiver<State>)>,
) -> response::Json<u64> {
    let state = state_rx.borrow().clone();
    response::Json(state.chain.get_balance(&state.address))
}

async fn handle_get_balance_with_address(
    extract::State((_, state_rx)): extract::State<(mpsc::Sender<Event>, watch::Receiver<State>)>,
    Path(address): Path<String>,
) -> response::Json<u64> {
    let state = state_rx.borrow().clone();
    response::Json(state.chain.get_balance(&Address { der: address }))
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
struct TransactionPayload {
    recipient: String,
    amount: u64,
}

async fn handle_post_transaction(
    extract::State((event_tx, _)): extract::State<(mpsc::Sender<Event>, watch::Receiver<State>)>,
    extract::Json(payload): extract::Json<TransactionPayload>,
) -> response::Json<bool> {
    response::Json(
        event_tx
            .send(Event::AddTransaction(
                PK {
                    der: payload.recipient,
                },
                payload.amount,
            ))
            .await
            .is_ok(),
    )
}

async fn handle_post_mine(
    extract::State((event_tx, _)): extract::State<(mpsc::Sender<Event>, watch::Receiver<State>)>,
) -> response::Json<bool> {
    response::Json(event_tx.send(Event::MineBlock).await.is_ok())
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
struct PeerPayload {
    ip: String,
}
async fn handle_post_peer(
    extract::State((event_tx, _)): extract::State<(mpsc::Sender<Event>, watch::Receiver<State>)>,
    extract::Json(payload): extract::Json<PeerPayload>,
) -> response::Json<bool> {
    response::Json(
        event_tx
            .send(Event::AddPeer(Peer { ip: payload.ip }))
            .await
            .is_ok(),
    )
}
