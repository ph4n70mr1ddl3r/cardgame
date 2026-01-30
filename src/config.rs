//! Configuration module for the poker server.
//!
//! This module handles loading and validating configuration from environment variables.
//! All configuration values have sensible defaults but can be overridden.

use serde::Deserialize;

/// Server configuration loaded from environment variables or defaults
#[derive(Debug, Clone, Deserialize)]
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
    /// Maximum database connections in pool
    pub db_max_connections: u32,
    /// Database connection timeout in seconds
    pub db_timeout_secs: u64,
}

/// Configuration errors
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Invalid value for {key}: {message}")]
    InvalidValue { key: String, message: String },

    #[error("Parse error for {key}: {message}")]
    ParseError { key: String, message: String },

    #[error("Out of range for {key}: {message}")]
    OutOfRange { key: String, message: String },

    #[error("Validation error: {0}")]
    ValidationError(String),
}

impl ConfigError {
    fn parse_owned(key: &str, message: String) -> Self {
        Self::ParseError {
            key: key.to_string(),
            message,
        }
    }

    fn out_of_range_owned(key: &str, message: String) -> Self {
        Self::OutOfRange {
            key: key.to_string(),
            message,
        }
    }

    fn validation(message: &str) -> Self {
        Self::ValidationError(message.to_string())
    }

    fn validation_owned<S: Into<String>>(message: S) -> Self {
        Self::ValidationError(message.into())
    }
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
            db_max_connections: 10,
            db_timeout_secs: 30,
        }
    }
}

impl Config {
    /// Loads configuration from environment variables or uses defaults.
    ///
    /// # Environment Variables
    ///
    /// - `POKER_SERVER_HOST` - Server address (default: "127.0.0.1")
    /// - `POKER_SERVER_PORT` - Server port (default: 8080)
    /// - `POKER_DATABASE_URL` - Database connection string (default: "sqlite:poker.db")
    /// - `POKER_MAX_TABLES` - Maximum concurrent tables (default: 5)
    /// - `POKER_SMALL_BLIND` - Small blind amount (default: 50)
    /// - `POKER_BIG_BLIND` - Big blind amount (default: 100)
    /// - `POKER_MIN_BUYIN_BB` - Minimum buy-in in big blinds (default: 20)
    /// - `POKER_MAX_BUYIN_BB` - Maximum buy-in in big blinds (default: 100)
    /// - `POKER_FAUCET_AMOUNT` - Top-up amount (default: 100)
    /// - `POKER_STARTING_CHIPS` - Starting chips for new players (default: 100)
    /// - `POKER_DB_MAX_CONNECTIONS` - Database pool size (default: 10)
    ///
    /// # Returns
    ///
    /// Returns validated configuration or error if values are invalid
    pub fn from_env() -> std::result::Result<Self, crate::error::PokerError> {
        let config = Self {
            server_host: Self::get_env_string("POKER_SERVER_HOST", "127.0.0.1"),
            server_port: Self::get_env("POKER_SERVER_PORT", 8080, Some(1024), Some(65535))
                .map_err(|e| crate::error::PokerError::game(e.to_string()))?,
            database_url: Self::get_env_string("POKER_DATABASE_URL", "sqlite:poker.db"),
            max_tables: Self::get_env("POKER_MAX_TABLES", 5, Some(1), Some(100))
                .map_err(|e| crate::error::PokerError::game(e.to_string()))?,
            disconnect_grace_period_secs: Self::get_env(
                "POKER_DISCONNECT_GRACE_SECS",
                30,
                Some(1),
                Some(3600),
            )
            .map_err(|e| crate::error::PokerError::game(e.to_string()))?,
            small_blind: Self::get_env("POKER_SMALL_BLIND", 50, Some(1), Some(10000))
                .map_err(|e| crate::error::PokerError::game(e.to_string()))?,
            big_blind: Self::get_env("POKER_BIG_BLIND", 100, Some(1), Some(10000))
                .map_err(|e| crate::error::PokerError::game(e.to_string()))?,
            min_buyin_bb: Self::get_env("POKER_MIN_BUYIN_BB", 20, Some(1), Some(1000))
                .map_err(|e| crate::error::PokerError::game(e.to_string()))?,
            max_buyin_bb: Self::get_env("POKER_MAX_BUYIN_BB", 100, Some(1), Some(1000))
                .map_err(|e| crate::error::PokerError::game(e.to_string()))?,
            faucet_amount: Self::get_env("POKER_FAUCET_AMOUNT", 100, Some(1), Some(100_000))
                .map_err(|e| crate::error::PokerError::game(e.to_string()))?,
            starting_chips: Self::get_env("POKER_STARTING_CHIPS", 100, Some(1), Some(100_000))
                .map_err(|e| crate::error::PokerError::game(e.to_string()))?,
            db_max_connections: Self::get_env("POKER_DB_MAX_CONNECTIONS", 10, Some(1), Some(100))
                .map_err(|e| crate::error::PokerError::game(e.to_string()))?,
            db_timeout_secs: Self::get_env("POKER_DB_TIMEOUT_SECS", 30, Some(1), Some(300))
                .map_err(|e| crate::error::PokerError::game(e.to_string()))?,
        };
        config
            .validate()
            .map_err(|e| crate::error::PokerError::game(e.to_string()))?;
        Ok(config)
    }

    fn get_env_string(key: &str, default: &str) -> String {
        std::env::var(key).unwrap_or_else(|_| default.to_string())
    }

    fn get_env<T>(
        key: &str,
        default: T,
        min: Option<T>,
        max: Option<T>,
    ) -> std::result::Result<T, ConfigError>
    where
        T: std::str::FromStr + std::fmt::Display + PartialOrd + Copy,
        T::Err: std::fmt::Display,
    {
        match std::env::var(key) {
            Ok(s) => {
                let value = s.parse::<T>().map_err(|e| {
                    ConfigError::parse_owned(key, format!("Failed to parse '{s}': {e}"))
                })?;

                if let Some(min_val) = min {
                    if value < min_val {
                        return Err(ConfigError::out_of_range_owned(
                            key,
                            format!("{value} is below minimum {min_val}"),
                        ));
                    }
                }
                if let Some(max_val) = max {
                    if value > max_val {
                        return Err(ConfigError::out_of_range_owned(
                            key,
                            format!("{value} exceeds maximum {max_val}"),
                        ));
                    }
                }
                Ok(value)
            }
            Err(_) => Ok(default),
        }
    }

    /// Validates configuration values for consistency and correctness.
    ///
    /// This method checks for:
    /// - Port validity (must be non-zero)
    /// - Positive values for chip amounts
    /// - Blind relationship (big blind >= small blind)
    /// - Buy-in constraints (max >= min)
    /// - Overflow detection
    /// - Non-zero values for essential settings
    ///
    /// # Errors
    ///
    /// Returns a `ConfigError` if any validation fails
    pub fn validate(&self) -> std::result::Result<(), ConfigError> {
        if self.server_port == 0 {
            return Err(ConfigError::validation(
                "POKER_SERVER_PORT must be in range 1-65535",
            ));
        }
        if self.max_tables == 0 {
            return Err(ConfigError::validation(
                "POKER_MAX_TABLES must be at least 1",
            ));
        }
        if self.small_blind <= 0 {
            return Err(ConfigError::validation(
                "POKER_SMALL_BLIND must be a positive number",
            ));
        }
        if self.big_blind <= 0 {
            return Err(ConfigError::validation(
                "POKER_BIG_BLIND must be a positive number",
            ));
        }
        if self.big_blind < self.small_blind {
            return Err(ConfigError::validation_owned(format!(
                "POKER_BIG_BLIND ({}) must be >= POKER_SMALL_BLIND ({})",
                self.big_blind, self.small_blind
            )));
        }
        if self.min_buyin_bb == 0 {
            return Err(ConfigError::validation(
                "POKER_MIN_BUYIN_BB must be at least 1 big blind",
            ));
        }
        if self.max_buyin_bb < self.min_buyin_bb {
            return Err(ConfigError::validation_owned(format!(
                "POKER_MAX_BUYIN_BB ({}) must be >= POKER_MIN_BUYIN_BB ({})",
                self.max_buyin_bb, self.min_buyin_bb
            )));
        }
        if self.faucet_amount <= 0 {
            return Err(ConfigError::validation(
                "POKER_FAUCET_AMOUNT must be a positive number",
            ));
        }
        if self.starting_chips <= 0 {
            return Err(ConfigError::validation(
                "POKER_STARTING_CHIPS must be a positive number",
            ));
        }
        if self.disconnect_grace_period_secs == 0 {
            return Err(ConfigError::validation(
                "POKER_DISCONNECT_GRACE_SECS must be at least 1 second",
            ));
        }
        if self.db_max_connections == 0 {
            return Err(ConfigError::validation(
                "POKER_DB_MAX_CONNECTIONS must be at least 1",
            ));
        }
        if self.db_timeout_secs == 0 {
            return Err(ConfigError::validation(
                "POKER_DB_TIMEOUT_SECS must be at least 1 second",
            ));
        }

        if self
            .big_blind
            .checked_mul(i64::from(self.max_buyin_bb))
            .is_none()
        {
            return Err(ConfigError::validation_owned(format!(
                "POKER_BIG_BLIND ({}) * POKER_MAX_BUYIN_BB ({}) would overflow i64",
                self.big_blind, self.max_buyin_bb
            )));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.server_host, "127.0.0.1");
        assert_eq!(config.server_port, 8080);
        assert_eq!(config.small_blind, 50);
        assert_eq!(config.big_blind, 100);
    }

    #[test]
    fn test_valid_config_passes_validation() {
        let config = Config {
            server_host: "0.0.0.0".to_string(),
            server_port: 9000,
            database_url: "sqlite:test.db".to_string(),
            max_tables: 10,
            disconnect_grace_period_secs: 60,
            small_blind: 25,
            big_blind: 50,
            min_buyin_bb: 20,
            max_buyin_bb: 200,
            faucet_amount: 500,
            starting_chips: 1000,
            db_max_connections: 20,
            db_timeout_secs: 60,
        };
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_invalid_port_zero() {
        let config = Config {
            server_port: 0,
            ..Default::default()
        };
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_invalid_max_tables_zero() {
        let config = Config {
            max_tables: 0,
            ..Default::default()
        };
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_negative_small_blind() {
        let config = Config {
            small_blind: -10,
            ..Default::default()
        };
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_negative_big_blind() {
        let config = Config {
            big_blind: -10,
            ..Default::default()
        };
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_big_blind_less_than_small_blind() {
        let config = Config {
            big_blind: 25,
            small_blind: 50,
            ..Default::default()
        };
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_max_buyin_less_than_min_buyin() {
        let config = Config {
            min_buyin_bb: 100,
            max_buyin_bb: 50,
            ..Default::default()
        };
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_negative_faucet_amount() {
        let config = Config {
            faucet_amount: -100,
            ..Default::default()
        };
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_negative_starting_chips() {
        let config = Config {
            starting_chips: -500,
            ..Default::default()
        };
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_overflow_detection() {
        let config = Config {
            big_blind: i64::MAX / 2,
            max_buyin_bb: 3,
            ..Default::default()
        };
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_zero_disconnect_grace_period() {
        let config = Config {
            disconnect_grace_period_secs: 0,
            ..Default::default()
        };
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_zero_db_connections() {
        let config = Config {
            db_max_connections: 0,
            ..Default::default()
        };
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_zero_db_timeout() {
        let config = Config {
            db_timeout_secs: 0,
            ..Default::default()
        };
        assert!(config.validate().is_err());
    }
}
