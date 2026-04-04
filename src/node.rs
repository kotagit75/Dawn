use openssl::error::ErrorStack;

use crate::util::key::{SK, generate_pk_and_sk};

const NODE_KEY_BITS: u32 = 512;

pub fn load_key() -> Result<SK, ErrorStack> {
    generate_pk_and_sk(NODE_KEY_BITS).map(|(_, sk)| sk)
}
