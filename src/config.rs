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
    pub starting_chips: i64,
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
            starting_chips: 100,
        }
    }
}

impl Config {
    pub fn from_env() -> Self {
        let mut config = Self {
            server_host: Self::get_env_string("POKER_SERVER_HOST", "127.0.0.1"),
            server_port: Self::get_env_u16("POKER_SERVER_PORT", 8080),
            database_url: Self::get_env_string("POKER_DATABASE_URL", "sqlite:poker.db"),
            max_tables: Self::get_env_usize("POKER_MAX_TABLES", 5),
            disconnect_grace_period_secs: Self::get_env_u64("POKER_DISCONNECT_GRACE_SECS", 30),
            small_blind: Self::get_env_i64("POKER_SMALL_BLIND", 50),
            big_blind: Self::get_env_i64("POKER_BIG_BLIND", 100),
            min_buyin_bb: Self::get_env_u32("POKER_MIN_BUYIN_BB", 20),
            max_buyin_bb: Self::get_env_u32("POKER_MAX_BUYIN_BB", 100),
            faucet_amount: Self::get_env_i64("POKER_FAUCET_AMOUNT", 100),
            starting_chips: Self::get_env_i64("POKER_STARTING_CHIPS", 100),
        };
        if let Err(e) = config.validate() {
            eprintln!(
                "Config validation error: {}, using default configuration",
                e
            );
            config = Self::default();
        }
        config
    }

    fn get_env_string(key: &str, default: &str) -> String {
        std::env::var(key).unwrap_or_else(|_| default.to_string())
    }

    fn get_env_u16(key: &str, default: u16) -> u16 {
        std::env::var(key)
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(default)
    }

    fn get_env_u32(key: &str, default: u32) -> u32 {
        std::env::var(key)
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(default)
    }

    fn get_env_usize(key: &str, default: usize) -> usize {
        std::env::var(key)
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(default)
    }

    fn get_env_u64(key: &str, default: u64) -> u64 {
        std::env::var(key)
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(default)
    }

    fn get_env_i64(key: &str, default: i64) -> i64 {
        std::env::var(key)
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(default)
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.server_port == 0 {
            return Err(format!(
                "Invalid POKER_SERVER_PORT ({}): must be > 0",
                self.server_port
            ));
        }
        if self.max_tables == 0 {
            return Err(format!(
                "Invalid POKER_MAX_TABLES ({}): must be > 0",
                self.max_tables
            ));
        }
        if self.small_blind <= 0 {
            return Err(format!(
                "Invalid POKER_SMALL_BLIND ({}): must be > 0",
                self.small_blind
            ));
        }
        if self.big_blind <= 0 {
            return Err(format!(
                "Invalid POKER_BIG_BLIND ({}): must be > 0",
                self.big_blind
            ));
        }
        if self.big_blind < self.small_blind {
            return Err(format!(
                "Invalid blind configuration: POKER_BIG_BLIND ({}) must be >= POKER_SMALL_BLIND ({})",
                self.big_blind, self.small_blind
            ));
        }
        if self.min_buyin_bb == 0 {
            return Err(format!(
                "Invalid POKER_MIN_BUYIN_BB ({}): must be > 0",
                self.min_buyin_bb
            ));
        }
        if self.max_buyin_bb < self.min_buyin_bb {
            return Err(format!(
                "Invalid buy-in configuration: POKER_MAX_BUYIN_BB ({}) must be >= POKER_MIN_BUYIN_BB ({})",
                self.max_buyin_bb, self.min_buyin_bb
            ));
        }
        if self.faucet_amount <= 0 {
            return Err(format!(
                "Invalid POKER_FAUCET_AMOUNT ({}): must be > 0",
                self.faucet_amount
            ));
        }
        if self.starting_chips <= 0 {
            return Err(format!(
                "Invalid POKER_STARTING_CHIPS ({}): must be > 0",
                self.starting_chips
            ));
        }
        Ok(())
    }
}
