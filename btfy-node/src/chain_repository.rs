use std::io::Error;

use tokio::io;

use btfy_core::chain::Chain;

pub trait ChainRepository {
    fn save(&self, chain: &Chain) -> Result<(), Error>;
    fn load(&self) -> Result<Chain, Error>;
    fn can_load(&self) -> bool;
    fn load_or_init(&self) -> Result<Chain, Error> {
        if self.can_load() {
            self.load()
        } else {
            Ok(Chain::new())
        }
    }
}

pub struct FileChainRepository {
    path: String,
}
impl FileChainRepository {
    pub fn new(path: &str) -> Self {
        Self {
            path: path.to_string(),
        }
    }
}

impl ChainRepository for FileChainRepository {
    fn save(&self, chain: &Chain) -> Result<(), Error> {
        let buf = bitcode::encode(chain);
        std::fs::write(self.path.clone(), buf)
    }
    fn load(&self) -> Result<Chain, Error> {
        std::fs::read(self.path.clone()).and_then(|s| bitcode::decode(&s).map_err(io::Error::other))
    }
    fn can_load(&self) -> bool {
        std::fs::metadata(self.path.clone()).is_ok()
    }
}
