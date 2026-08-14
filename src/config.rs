#[derive(Clone)]
pub struct Config {
    pub mining: bool,
    pub peer: Vec<String>,
    pub api_port: u16,
    pub p2p_port: u16,
    pub beacon_timeout: u64,
    pub beacon_cmd: Vec<String>,
    pub vdf_difficulty: u64,
}
