pub mod beacon;
pub mod blockchain;
pub mod node;
pub mod state;
pub mod update;
pub mod util;

fn main() {
    let Ok(sk) = node::load_key() else {
        return;
    };
    let Ok(state) = state::State::new(sk) else {
        return;
    };
    println!("address: {:?}", state.address)
}
