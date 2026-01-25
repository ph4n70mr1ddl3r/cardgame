use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub id: i64,
    pub username: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub chips: i64,
    pub hands_played: i64,
    pub hands_won: i64,
}

impl Player {
    pub fn new(id: i64, username: String, password_hash: String) -> Self {
        Self {
            id,
            username,
            password_hash,
            chips: 100, // Start with 100 play money chips
            hands_played: 0,
            hands_won: 0,
        }
    }

    pub fn can_top_up(&self) -> bool {
        self.chips < 100
    }

    pub fn top_up(&mut self, faucet_amount: i64) {
        if self.can_top_up() {
            self.chips = faucet_amount;
        }
    }

    pub fn deduct_chips(&mut self, amount: i64) -> bool {
        if amount >= 0 && self.chips >= amount {
            self.chips -= amount;
            true
        } else {
            false
        }
    }

    pub fn add_chips(&mut self, amount: i64) {
        if amount >= 0 {
            self.chips += amount;
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerSession {
    pub player_id: i64,
    pub username: String,
    pub table_id: Option<i64>,
    pub seat_index: Option<usize>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player_creation() {
        let player = Player::new(1, "test_user".to_string(), "hash123".to_string());
        assert_eq!(player.chips, 100);
        assert_eq!(player.hands_played, 0);
        assert_eq!(player.hands_won, 0);
    }

    #[test]
    fn test_top_up() {
        let mut player = Player::new(1, "test".to_string(), "hash".to_string());
        player.chips = 50;

        assert!(player.can_top_up());
        player.top_up(100);
        assert_eq!(player.chips, 100);

        // Cannot top up when chips >= 100
        assert!(!player.can_top_up());
        player.top_up(100);
        assert_eq!(player.chips, 100); // Unchanged
    }

    #[test]
    fn test_chip_operations() {
        let mut player = Player::new(1, "test".to_string(), "hash".to_string());

        assert!(player.deduct_chips(50));
        assert_eq!(player.chips, 50);

        assert!(!player.deduct_chips(100)); // Insufficient chips
        assert_eq!(player.chips, 50); // Unchanged

        player.add_chips(75);
        assert_eq!(player.chips, 125);
    }
}
