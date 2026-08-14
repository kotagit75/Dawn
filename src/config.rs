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

impl Default for Config {
    fn default() -> Self {
        Self {
            vdf_difficulty: 10,
            beacon_cmd: Vec::new(),
            mining: false,
            peer: Vec::new(),
            api_port: 8080,
            p2p_port: 8081,
            beacon_timeout: 10,
        }
    }
}
