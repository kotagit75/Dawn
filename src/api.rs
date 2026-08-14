use std::{
    io::Error,
    net::{Ipv4Addr, SocketAddr},
};

use axum::{
    Router,
    extract::{self, Path},
    http::StatusCode,
    response,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use tokio::sync::{mpsc, oneshot, watch};

use crate::{
    blockchain::{address::Address, chain::Chain},
    event::{Event, UpdateResult, command::Command},
    p2p::Peer,
    state::State,
    util::key::PK,
};

type AppState = (mpsc::Sender<Command>, watch::Receiver<State>);

fn read_state<T, F>(state_rx: &watch::Receiver<State>, f: F) -> T
where
    F: FnOnce(&State) -> T,
{
    let state = state_rx.borrow();
    f(&state)
}

fn json_query<T, F>(state_rx: &watch::Receiver<State>, f: F) -> response::Json<T>
where
    T: Serialize,
    F: FnOnce(&State) -> T,
{
    response::Json(read_state(state_rx, f))
}

async fn dispatch_event(
    event_tx: &mpsc::Sender<Command>,
    event: Event,
) -> Result<UpdateResult, StatusCode> {
    let (response_tx, response_rx) = oneshot::channel();
    event_tx
        .send(Command::ApiRequest(event, response_tx))
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    response_rx
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn init_api(
    event_tx: mpsc::Sender<Command>,
    state_rx: watch::Receiver<State>,
    api_port: u16,
) -> Result<(), Error> {
    let app = Router::new()
        .route("/health", get(handle_query_health))
        .route("/address", get(handle_query_address))
        .route("/peers", get(handle_query_peers))
        .route("/chain", get(handle_query_chain))
        .route("/balance", get(handle_query_balance))
        .route("/balance/{address}", get(handle_query_balance_with_address))
        .route("/tx", post(handle_command_transaction))
        .route("/peer", post(handle_command_peer))
        .with_state((event_tx, state_rx));
    let addr = SocketAddr::new(std::net::IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), api_port);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    info!("API server is running on http://{}/", addr);
    axum::serve(listener, app).await
}

async fn handle_query_health(extract::State((_, _)): extract::State<AppState>) -> &'static str {
    "ok"
}

async fn handle_query_address(extract::State((_, state_rx)): extract::State<AppState>) -> String {
    read_state(&state_rx, |state| state.address.der.clone())
}

async fn handle_query_peers(
    extract::State((_, state_rx)): extract::State<AppState>,
) -> response::Json<Vec<Peer>> {
    json_query(&state_rx, |state| state.peers.clone())
}

async fn handle_query_chain(
    extract::State((_, state_rx)): extract::State<AppState>,
) -> response::Json<Chain> {
    json_query(&state_rx, |state| state.chain.clone())
}

async fn handle_query_balance(
    extract::State((_, state_rx)): extract::State<AppState>,
) -> response::Json<u64> {
    json_query(&state_rx, |state| state.chain.get_balance(&state.address))
}

async fn handle_query_balance_with_address(
    extract::State((_, state_rx)): extract::State<AppState>,
    Path(address): Path<String>,
) -> response::Json<u64> {
    json_query(&state_rx, |state| {
        state.chain.get_balance(&Address { der: address })
    })
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
struct TransactionPayload {
    recipient: String,
    send_amount: u64,
    fee: u64,
}
async fn handle_command_transaction(
    extract::State((event_tx, _)): extract::State<AppState>,
    extract::Json(payload): extract::Json<TransactionPayload>,
) -> Result<response::Json<UpdateResult>, StatusCode> {
    let result = dispatch_event(
        &event_tx,
        Event::AddTransaction(
            PK {
                der: payload.recipient,
            },
            payload.send_amount,
            payload.fee,
        ),
    )
    .await?;
    Ok(response::Json(result))
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
struct PeerPayload {
    addr: String,
}
async fn handle_command_peer(
    extract::State((event_tx, _)): extract::State<AppState>,
    extract::Json(payload): extract::Json<PeerPayload>,
) -> Result<response::Json<UpdateResult>, StatusCode> {
    let result = dispatch_event(&event_tx, Event::AddPeer(Peer::new(&payload.addr))).await?;
    Ok(response::Json(result))
}
