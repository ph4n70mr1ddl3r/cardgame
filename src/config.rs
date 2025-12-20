use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub server_host: String,
    pub server_port: u16,
    pub database_url: String,
    pub max_tables: usize,
    pub disconnect_grace_period_secs: u64,
    pub small_blind: f64,
    pub big_blind: f64,
    pub min_buyin_bb: u32,
    pub max_buyin_bb: u32,
    pub faucet_amount: i64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            server_host: "127.0.0.1".to_string(),
            server_port: 8080,
            database_url: "sqlite:poker.db".to_string(),
            max_tables: 5,
            disconnect_grace_period_secs: 30,
            small_blind: 0.5,
            big_blind: 1.0,
            min_buyin_bb: 20,
            max_buyin_bb: 100,
            faucet_amount: 100,
        }
    }
}

impl Config {
    pub fn from_env() -> Self {
        // Future: load from environment variables or config file
        Self::default()
    }
}
