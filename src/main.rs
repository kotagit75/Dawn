pub mod beacon;
pub mod blockchain;
pub mod node;
pub mod state;
pub mod util;

fn main() {
    let Ok(sk) = node::load_key() else {
        return;
    };
    println!("address: {:?}", sk.to_pk())
}
