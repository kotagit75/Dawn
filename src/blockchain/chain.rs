use bitcode::{Decode, Encode};
use serde::{Deserialize, Serialize};

use crate::{
    beacon::{BeaconCache, BeaconKey, is_valid_beacon},
    blockchain::{
        block::{Block, genesis_block},
        validation::is_valid_new_block,
    },
};

// For blocks older than CHECKPOINT_DEPTH, temperature verification is omitted.
pub const CHECKPOINT_DEPTH: usize = 600;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Encode, Decode)]
pub struct Chain {
    pub blocks: Vec<Block>,
}

impl Default for Chain {
    fn default() -> Self {
        Self::new()
    }
}

impl Chain {
    pub fn new() -> Self {
        Self {
            blocks: vec![genesis_block()],
        }
    }

    pub fn get_latest_block(&self) -> Block {
        match self.blocks.last() {
            Some(block) => block.clone(),
            None => genesis_block(),
        }
    }

    pub fn replace(&self, new_chain: Chain, cache: &dyn BeaconCache) -> Self {
        if new_chain.is_valid(cache) && new_chain.blocks.len() > self.blocks.len() {
            Self {
                blocks: new_chain.blocks,
            }
        } else {
            self.clone()
        }
    }

    pub fn get_block_depth(&self, block: &Block) -> usize {
        self.blocks
            .iter()
            .rev()
            .take_while(|b| b.hash != block.previous_hash)
            .count()
    }

    fn add_block_without_validation(&self, block: Block) -> Self {
        Self {
            blocks: self.blocks.iter().chain([&block]).cloned().collect(),
        }
    }

    pub fn add_block(
        &self,
        block: Block,
        i_generated: bool,
        generated_now: bool,
        cache: &dyn BeaconCache,
    ) -> (Self, bool) {
        let previous_block = self.get_latest_block();
        let beacon_ok = (!i_generated && generated_now)
            .then(|| {
                cache
                    .get(&BeaconKey::new(&previous_block.hash, block.timestamp))
                    .map(|beacon| is_valid_beacon(&beacon, &block.beacon))
                    .unwrap_or(false)
            })
            .unwrap_or(true);
        let is_valid_new_block = is_valid_new_block(
            &block,
            &previous_block,
            &self.get_unspent_transactions().0,
            self.get_block_depth(&block),
            cache,
        );

        if is_valid_new_block && beacon_ok {
            (self.add_block_without_validation(block), true)
        } else {
            (self.clone(), false)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::beacon::{Beacon, InMemoryBeaconCache};
    use crate::blockchain::block::{Block, genesis_block};
    use crate::blockchain::transaction::Transaction;
    use crate::util::signature::SignatureWrapper;

    fn dummy_block(prev: &Block, txs: Vec<Transaction>, beacon: i32) -> Block {
        Block {
            index: prev.index + 1,
            timestamp: prev.timestamp + 1,
            transactions: txs,
            beacon: Beacon {
                values: vec![beacon],
            },
            vdf_solution: vec![],
            previous_hash: prev.hash,
            issuer: prev.issuer.clone(),
            signature: SignatureWrapper::default(),
            hash: [prev.index as u8 + 1; 32],
        }
    }

    #[test]
    fn new_has_only_genesis() {
        let c = Chain::new();
        assert_eq!(c.blocks.len(), 1);
        assert_eq!(c.get_latest_block(), genesis_block());
    }

    #[test]
    fn add_block_rejects_invalid_block() {
        let c = Chain::new();
        let bad = dummy_block(&c.get_latest_block(), vec![], 1);
        let cache = InMemoryBeaconCache::new();
        let (next, changed) = c.add_block(bad, false, false, &cache);
        assert!(!changed);
        assert_eq!(next, c);
    }

    #[test]
    fn replace_rejects_invalid_longer_chain() {
        let base = Chain::new();
        let g = genesis_block();
        let longer_but_invalid = Chain {
            blocks: vec![g.clone(), dummy_block(&g, vec![], 1)],
        };
        let cache = InMemoryBeaconCache::new();
        assert_eq!(base.replace(longer_but_invalid, &cache), base);
    }

    #[test]
    fn get_block_depth_counts_from_tip() {
        let g = genesis_block();
        let b1 = dummy_block(&g, vec![], 1);
        let b2 = dummy_block(&b1, vec![], 2);
        let b3 = dummy_block(&b2, vec![], 3);
        let c = Chain {
            blocks: vec![g.clone(), b1.clone(), b2.clone(), b3.clone()],
        };

        assert_eq!(c.get_block_depth(&b3), 1);
        assert_eq!(c.get_block_depth(&b2), 2);
        assert_eq!(c.get_block_depth(&b1), 3);
        assert_eq!(c.get_block_depth(&g), 4);
    }
}
