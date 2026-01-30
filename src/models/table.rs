use crate::error::{PokerError, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Table {
    pub id: i64,
    pub name: String,
    pub small_blind: i64,
    pub big_blind: i64,
    pub max_players: i32,
    pub current_players: usize,
}

impl Table {
    /// Creates a new table with standard maximum of 2 players (heads-up).
    pub fn new(id: i64, name: String, small_blind: i64, big_blind: i64) -> Result<Self> {
        Self::with_max_players(id, name, small_blind, big_blind, 2)
    }

    /// Creates a new table with custom maximum player count.
    pub fn with_max_players(
        id: i64,
        name: String,
        small_blind: i64,
        big_blind: i64,
        max_players: i32,
    ) -> Result<Self> {
        if name.trim().is_empty() {
            return Err(PokerError::game("Table name cannot be empty"));
        }
        if name.len() > 100 {
            return Err(PokerError::game("Table name cannot exceed 100 characters"));
        }
        if small_blind <= 0 {
            return Err(PokerError::game("Small blind must be positive"));
        }
        if big_blind <= 0 {
            return Err(PokerError::game("Big blind must be positive"));
        }
        if big_blind < small_blind {
            return Err(PokerError::game("Big blind must be >= small blind"));
        }
        if max_players < 2 {
            return Err(PokerError::game("Max players must be at least 2"));
        }

        Ok(Self {
            id,
            name,
            small_blind,
            big_blind,
            max_players,
            current_players: 0,
        })
    }

    #[must_use]
    pub fn is_full(&self) -> bool {
        self.current_players >= self.max_players as usize
    }
    #[must_use]
    pub fn can_join(&self) -> bool {
        !self.is_full()
    }

    pub fn validate_buyin(&self, buyin: i64, min_buyin_bb: u32, max_buyin_bb: u32) -> Result<()> {
        if buyin < 0 {
            return Err(PokerError::game("Buy-in cannot be negative"));
        }
        let min_buyin = self.big_blind * i64::from(min_buyin_bb);
        let max_buyin = self.big_blind * i64::from(max_buyin_bb);
        if buyin < min_buyin {
            return Err(PokerError::game(format!(
                "Buy-in must be at least {min_buyin_bb} big blinds"
            )));
        }
        if buyin > max_buyin {
            return Err(PokerError::game(format!(
                "Buy-in cannot exceed {max_buyin_bb} big blinds"
            )));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_table_creation() {
        let table = Table::new(1, "Table 1".to_string(), 50, 100).unwrap();
        assert_eq!(table.max_players, 2);
        assert_eq!(table.current_players, 0);
        assert!(table.can_join());
    }

    #[test]
    fn test_table_full() {
        let mut table = Table::new(1, "Table 1".to_string(), 50, 100).unwrap();
        table.current_players = 2;
        assert!(table.is_full());
        assert!(!table.can_join());
    }

    #[test]
    fn test_table_name_validation() {
        assert!(Table::new(1, "".to_string(), 50, 100).is_err());
        assert!(Table::new(1, "   ".to_string(), 50, 100).is_err());
        let long_name = "a".repeat(101);
        assert!(Table::new(1, long_name, 50, 100).is_err());
    }

    #[test]
    fn test_invalid_blinds() {
        assert!(Table::new(1, "Table".to_string(), -1, 100).is_err());
        assert!(Table::new(1, "Table".to_string(), 50, -1).is_err());
        assert!(Table::new(1, "Table".to_string(), 100, 50).is_err());
    }
}
