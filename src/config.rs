use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub server_host: String,
    pub server_port: u16,
    pub database_url: String,
    pub max_tables: usize,
    pub disconnect_grace_period_secs: u64,
    pub small_blind: i64,
    pub big_blind: i64,
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
            small_blind: 50,
            big_blind: 100,
            min_buyin_bb: 20,
            max_buyin_bb: 100,
            faucet_amount: 100,
        }
    }
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            server_host: std::env::var("POKER_SERVER_HOST")
                .unwrap_or_else(|_| "127.0.0.1".to_string()),
            server_port: std::env::var("POKER_SERVER_PORT")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(8080),
            database_url: std::env::var("POKER_DATABASE_URL")
                .unwrap_or_else(|_| "sqlite:poker.db".to_string()),
            max_tables: std::env::var("POKER_MAX_TABLES")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(5),
            disconnect_grace_period_secs: std::env::var("POKER_DISCONNECT_GRACE_SECS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(30),
            small_blind: std::env::var("POKER_SMALL_BLIND")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(50),
            big_blind: std::env::var("POKER_BIG_BLIND")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(100),
            min_buyin_bb: std::env::var("POKER_MIN_BUYIN_BB")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(20),
            max_buyin_bb: std::env::var("POKER_MAX_BUYIN_BB")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(100),
            faucet_amount: std::env::var("POKER_FAUCET_AMOUNT")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(100),
        }
    }
}
