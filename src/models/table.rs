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
    pub fn new(id: i64, name: String, small_blind: i64, big_blind: i64) -> Self {
        Self {
            id,
            name,
            small_blind,
            big_blind,
            max_players: 2,
            current_players: 0,
        }
    }

    pub fn is_full(&self) -> bool {
        self.current_players >= self.max_players as usize
    }

    pub fn can_join(&self) -> bool {
        !self.is_full()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_table_creation() {
        let table = Table::new(1, "Table 1".to_string(), 50, 100);
        assert_eq!(table.max_players, 2);
        assert_eq!(table.current_players, 0);
        assert!(table.can_join());
    }

    #[test]
    fn test_table_full() {
        let mut table = Table::new(1, "Table 1".to_string(), 50, 100);
        table.current_players = 2;
        assert!(table.is_full());
        assert!(!table.can_join());
    }
}
