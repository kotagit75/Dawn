use crate::{
    p2p::{P2PMessage, Peer},
    state::State,
    update::effect::{Effect, map_effect},
};

pub fn handle_add_peer(state: State, peer: Peer) -> (State, Effect) {
    let (state, changed) = state.add_peer(&peer);
    if changed {
        info!("added peer: {}", peer.ip);
    } else {
        error!("peer already exists: {}", peer.ip);
    }
    (
        state,
        map_effect(|| Effect::Broadcast(P2PMessage::QueryPeers), changed),
    )
}

pub fn handle_remove_peers(state: State, peers: Vec<Peer>) -> (State, Effect) {
    info!(
        "remove peers: {:?}",
        peers.iter().map(|peer| peer.ip.to_string())
    );
    (state.remove_peers(&peers).0, Effect::None)
}
