use sha2::{Digest, Sha256};

pub type Hashed = [u8; 32];

pub fn hash(data: &[u8]) -> Hashed {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().into()
}
