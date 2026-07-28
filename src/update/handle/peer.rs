use crate::{
    p2p::{P2PMessage, Peer},
    state::{State, add_peers},
    update::effect::{Effect, map_effect},
};

pub fn handle_add_peer(state: State, new_peer: Peer) -> (State, Effect) {
    let (new_state, added) = add_peers(state, &[new_peer]);
    (
        new_state,
        map_effect(|| Effect::Broadcast(P2PMessage::QueryPeers), added),
    )
}

pub fn handle_remove_peers(state: State, remove_peers: Vec<Peer>) -> (State, Effect) {
    let remove_peers_ip = remove_peers.iter().map(|peer| peer.ip.to_string());
    if !remove_peers.is_empty() {
        debug!("remove peers: {:?}", remove_peers_ip);
    }
    let mut peers = state.peers.clone();
    peers.retain(|peer| !remove_peers.contains(peer));
    (State { peers, ..state }, Effect::None)
}
