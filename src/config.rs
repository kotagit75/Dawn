use std::sync::LazyLock;

use clap::Parser;

pub const VDF_DIFFICULTY: u64 = 5295676;

#[derive(Parser, Debug, Clone)]
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

    /// The port to listen on for the P2P network
    #[arg(long, default_value = "62697")]
    pub p2p_port: u16,

    /// The timeout for API requests in seconds
    #[arg(short, long, default_value = "5")]
    pub beacon_timeout: u64,

    /// Beacon provider command to run over stdio
    #[arg(long = "beacon-cmd", num_args = 1.., value_name = "CMD")]
    pub beacon_cmd: Vec<String>,

    /// For testing only: vdf difficulty
    #[arg(long)]
    pub vdf_difficulty: Option<u64>,
}
#[derive(Debug, Clone)]
pub struct InternalConfig {
    pub vdf_difficulty: u64,
}
pub struct Config {
    pub args: Args,
    pub internal_config: InternalConfig,
}

pub static CONFIG: LazyLock<Config> = LazyLock::new(|| {
    let args = Args::parse();

    Config {
        args: args.clone(),
        internal_config: InternalConfig {
            vdf_difficulty: args.vdf_difficulty.unwrap_or(VDF_DIFFICULTY),
        },
    }
});
