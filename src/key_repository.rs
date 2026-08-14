use std::io::Error;

use crate::util::key::{SK, generate_sk};

const NODE_KEY_BITS: usize = 2048;

pub trait KeyRepository {
    fn save(&self, sk: &SK) -> Result<(), Error>;
    fn load(&self) -> Result<SK, Error>;
    fn can_load(&self) -> bool;
    fn load_or_init(&self) -> Result<SK, Error> {
        if self.can_load() {
            self.load()
        } else {
            let key = generate_sk(NODE_KEY_BITS);
            self.save(&key)?;
            Ok(key)
        }
    }
}

pub struct FileKeyRepository {
    path: String,
}
impl FileKeyRepository {
    pub fn new(path: &str) -> Self {
        Self {
            path: path.to_string(),
        }
    }
}

impl KeyRepository for FileKeyRepository {
    fn save(&self, sk: &SK) -> Result<(), Error> {
        std::fs::write(self.path.clone(), &sk.der)
    }
    fn load(&self) -> Result<SK, Error> {
        std::fs::read_to_string(self.path.clone()).map(|der| SK { der })
    }
    fn can_load(&self) -> bool {
        std::fs::metadata(self.path.clone()).is_ok()
    }
}
