use std::sync::LazyLock;

use clap::Parser;

pub const P2P_PORT: u16 = 62697;
pub const VDF_DIFFICULTY: u64 = 5295676;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Whether to mine blocks
    #[arg(short, long)]
    pub mining: bool,

    /// The IP address to add to the peer list
    #[arg(short, long)]
    pub peer: Option<String>,

    /// The port to listen on for the API
    #[arg(short, long, default_value = "8080")]
    pub api_port: u16,

    /// The timeout for API requests in seconds
    #[arg(short, long, default_value = "5")]
    pub beacon_timeout: u64,

    /// Beacon provider command to run over stdio
    #[arg(long = "beacon-cmd", num_args = 1.., value_name = "CMD")]
    pub beacon_cmd: Vec<String>,
}
#[derive(Debug, Clone)]
pub struct InternalConfig {
    pub p2p_port: u16,
    pub vdf_difficulty: u64,
}
pub struct Config {
    pub args: Args,
    pub internal_config: InternalConfig,
}

pub static CONFIG: LazyLock<Config> = LazyLock::new(|| {
    let args = Args::parse();

    Config {
        args,
        internal_config: InternalConfig {
            p2p_port: P2P_PORT,
            vdf_difficulty: VDF_DIFFICULTY,
        },
    }
});
