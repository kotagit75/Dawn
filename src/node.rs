use std::io;

use crate::{
    blockchain::chain::Chain,
    util::key::{SK, generate_sk},
};

const NODE_KEY_BITS: usize = 512;

const NODE_GITIGNORE_FILE_NAME: &str = ".gitignore";
const NODE_KEY_FILE_NAME: &str = "key.der";
const NODE_CHAIN_FILE_NAME: &str = "chain";

macro_rules! node_path {
    () => {
        "node"
    };
    ($file:expr) => {
        format!("{}/{}", node_path!(), $file)
    };
}

fn create_node_dir() -> Result<(), io::Error> {
    std::fs::create_dir(node_path!())
}

fn create_gitignore() -> Result<(), io::Error> {
    std::fs::write(
        node_path!(NODE_GITIGNORE_FILE_NAME),
        format!("{}\n", NODE_KEY_FILE_NAME),
    )
}

pub fn load_or_generate_key() -> Result<SK, io::Error> {
    debug!("create node directory");
    if std::fs::metadata(node_path!()).is_err() {
        create_node_dir()
            .inspect_err(|err| error!("failed to create the node directory: {:?}", err))?;
    }
    debug!("create gitignore");
    if std::fs::metadata(node_path!(NODE_GITIGNORE_FILE_NAME)).is_err() {
        create_gitignore()
            .inspect_err(|err| error!("failed to create the gitignore file: {:?}", err))?;
    }

    if std::fs::metadata(node_path!(NODE_KEY_FILE_NAME)).is_ok() {
        debug!("read node key");
        read_key().inspect_err(|err| {
            error!("failed to read node key: {}", err);
        })
    } else {
        debug!("generate node key");
        let sk = generate_key();
        save_key(&sk).inspect_err(|err| {
            error!("failed to save node key: {}", err);
        })?;
        Ok(sk)
    }
}

pub fn generate_key() -> SK {
    generate_sk(NODE_KEY_BITS)
}

pub fn read_key() -> Result<SK, io::Error> {
    std::fs::read_to_string(node_path!(NODE_KEY_FILE_NAME)).map(|der| SK { der })
}

pub fn save_key(sk: &SK) -> Result<(), io::Error> {
    std::fs::write(node_path!(NODE_KEY_FILE_NAME), &sk.der)
}

pub fn load_or_generate_chain() -> Result<Chain, io::Error> {
    if std::fs::metadata(node_path!(NODE_CHAIN_FILE_NAME)).is_err() {
        debug!("generate chain");
        let chain = Chain::new();
        save_chain(&chain).inspect_err(|e| {
            error!("failed to save chain: {}", e);
        })?;
        return Ok(chain);
    }
    load_chain()
}
pub fn load_chain() -> Result<Chain, io::Error> {
    std::fs::read(node_path!(NODE_CHAIN_FILE_NAME))
        .and_then(|s| bitcode::decode(&s).map_err(io::Error::other))
}
pub fn save_chain(chain: &Chain) -> Result<(), io::Error> {
    let buf = bitcode::encode(chain);
    std::fs::write(node_path!(NODE_CHAIN_FILE_NAME), buf)
}
