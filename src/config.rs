use serde::{Deserialize, Serialize};

/// Server configuration loaded from environment variables or defaults
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    /// Server listening address
    pub server_host: String,
    /// Server listening port
    pub server_port: u16,
    /// Database connection URL
    pub database_url: String,
    /// Maximum number of concurrent tables
    pub max_tables: usize,
    /// Grace period in seconds before auto-fold on disconnect
    pub disconnect_grace_period_secs: u64,
    /// Small blind amount in chips
    pub small_blind: i64,
    /// Big blind amount in chips
    pub big_blind: i64,
    /// Minimum buy-in in big blinds
    pub min_buyin_bb: u32,
    /// Maximum buy-in in big blinds
    pub max_buyin_bb: u32,
    /// Amount to top-up when using the faucet
    pub faucet_amount: i64,
    /// Starting chips for new players
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
    pub fn from_env() -> Result<Self, crate::error::PokerError> {
        let config = Self {
            server_host: Self::get_env_string("POKER_SERVER_HOST", "127.0.0.1"),
            server_port: Self::get_env("POKER_SERVER_PORT", 8080, Some(1024), Some(65535)),
            database_url: Self::get_env_string("POKER_DATABASE_URL", "sqlite:poker.db"),
            max_tables: Self::get_env("POKER_MAX_TABLES", 5, Some(1), Some(100)),
            disconnect_grace_period_secs: Self::get_env(
                "POKER_DISCONNECT_GRACE_SECS",
                30,
                Some(1),
                Some(3600),
            ),
            small_blind: Self::get_env("POKER_SMALL_BLIND", 50, Some(1), Some(10000)),
            big_blind: Self::get_env("POKER_BIG_BLIND", 100, Some(1), Some(10000)),
            min_buyin_bb: Self::get_env("POKER_MIN_BUYIN_BB", 20, Some(1), Some(1000)),
            max_buyin_bb: Self::get_env("POKER_MAX_BUYIN_BB", 100, Some(1), Some(1000)),
            faucet_amount: Self::get_env("POKER_FAUCET_AMOUNT", 100, Some(1), Some(100000)),
            starting_chips: Self::get_env("POKER_STARTING_CHIPS", 100, Some(1), Some(100000)),
        };
        config.validate()?;
        Ok(config)
    }

    fn get_env_string(key: &str, default: &str) -> String {
        std::env::var(key).unwrap_or_else(|_| default.to_string())
    }

    fn get_env<T>(key: &str, default: T, min: Option<T>, max: Option<T>) -> T
    where
        T: std::str::FromStr + std::fmt::Display + PartialOrd + Copy,
        T::Err: std::fmt::Display,
    {
        std::env::var(key)
            .ok()
            .and_then(|s| {
                let value = s.parse::<T>().ok()?;
                if let Some(min_val) = min {
                    if value < min_val {
                        eprintln!(
                            "Warning: {} value {} is below minimum {}, using default {}",
                            key, value, min_val, default
                        );
                        return None;
                    }
                }
                if let Some(max_val) = max {
                    if value > max_val {
                        eprintln!(
                            "Warning: {} value {} exceeds maximum {}, using default {}",
                            key, value, max_val, default
                        );
                        return None;
                    }
                }
                Some(value)
            })
            .unwrap_or(default)
    }

    pub fn validate(&self) -> Result<(), crate::error::PokerError> {
        if self.server_port == 0 {
            return Err(crate::error::PokerError::Game(format!(
                "Invalid POKER_SERVER_PORT: {}, expected range 1-65535",
                self.server_port
            )));
        }
        if self.max_tables == 0 {
            return Err(crate::error::PokerError::Game(format!(
                "Invalid POKER_MAX_TABLES: {}, must be at least 1",
                self.max_tables
            )));
        }
        if self.small_blind <= 0 {
            return Err(crate::error::PokerError::Game(format!(
                "Invalid POKER_SMALL_BLIND: {}, must be a positive number",
                self.small_blind
            )));
        }
        if self.big_blind <= 0 {
            return Err(crate::error::PokerError::Game(format!(
                "Invalid POKER_BIG_BLIND: {}, must be a positive number",
                self.big_blind
            )));
        }
        if self.big_blind < self.small_blind {
            return Err(crate::error::PokerError::Game(format!(
                "Invalid blind configuration: POKER_BIG_BLIND ({}) must be >= POKER_SMALL_BLIND ({}), got BB {} < SB {}",
                self.big_blind, self.small_blind, self.big_blind, self.small_blind
            )));
        }
        if self.min_buyin_bb == 0 {
            return Err(crate::error::PokerError::Game(format!(
                "Invalid POKER_MIN_BUYIN_BB: {}, must be at least 1 big blind",
                self.min_buyin_bb
            )));
        }
        if self.max_buyin_bb < self.min_buyin_bb {
            return Err(crate::error::PokerError::Game(format!(
                "Invalid buy-in configuration: POKER_MAX_BUYIN_BB ({}) must be >= POKER_MIN_BUYIN_BB ({}), got max {} < min {}",
                self.max_buyin_bb, self.min_buyin_bb, self.max_buyin_bb, self.min_buyin_bb
            )));
        }
        if self.faucet_amount <= 0 {
            return Err(crate::error::PokerError::Game(format!(
                "Invalid POKER_FAUCET_AMOUNT: {}, must be a positive number",
                self.faucet_amount
            )));
        }
        if self.starting_chips <= 0 {
            return Err(crate::error::PokerError::Game(format!(
                "Invalid POKER_STARTING_CHIPS: {}, must be a positive number",
                self.starting_chips
            )));
        }
        if self.disconnect_grace_period_secs == 0 {
            return Err(crate::error::PokerError::Game(format!(
                "Invalid POKER_DISCONNECT_GRACE_SECS: {}, must be at least 1 second",
                self.disconnect_grace_period_secs
            )));
        }

        if self
            .big_blind
            .checked_mul(self.max_buyin_bb as i64)
            .is_none()
        {
            return Err(crate::error::PokerError::Game(format!(
                "Invalid configuration: POKER_BIG_BLIND ({}) * POKER_MAX_BUYIN_BB ({}) would overflow i64, please reduce one or both values",
                self.big_blind, self.max_buyin_bb
            )));
        }

        Ok(())
    }
}
