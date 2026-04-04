use serde::{Deserialize, Serialize};

use crate::blockchain::block::{Block, genesis_block};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
struct Chain {
    blocks: Vec<Block>,
}

impl Chain {
    pub fn new() -> Self {
        Self {
            blocks: vec![genesis_block()],
        }
    }
}
