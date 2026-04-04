use crate::{
    blockchain::{address::Address, chain::Chain},
    util::key::SK,
};

struct State {
    secret_key: SK,
    address: Address,
    chain: Chain,
}
