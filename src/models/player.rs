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
    pub fn new(id: i64, username: String, password_hash: String, starting_chips: i64) -> Self {
        Self {
            id,
            username,
            password_hash,
            chips: starting_chips,
            hands_played: 0,
            hands_won: 0,
        }
    }

    pub fn can_top_up(&self, threshold: i64) -> bool {
        self.chips < threshold
    }

    pub fn top_up(&mut self, faucet_amount: i64, threshold: i64) -> Result<(), String> {
        if !self.can_top_up(threshold) {
            return Err("Player already has enough chips to top up".to_string());
        }
        self.chips = faucet_amount;
        Ok(())
    }

    pub fn deduct_chips(&mut self, amount: i64) -> bool {
        if amount < 0 {
            return false;
        }
        if amount > self.chips {
            return false;
        }
        self.chips -= amount;
        true
    }

    pub fn add_chips(&mut self, amount: i64) -> bool {
        if amount < 0 {
            return false;
        }
        match self.chips.checked_add(amount) {
            Some(new_chips) => {
                self.chips = new_chips;
                true
            }
            None => false,
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
        let player = Player::new(1, "test_user".to_string(), "hash123".to_string(), 100);
        assert_eq!(player.chips, 100);
        assert_eq!(player.hands_played, 0);
        assert_eq!(player.hands_won, 0);
    }

    #[test]
    fn test_top_up() {
        let mut player = Player::new(1, "test".to_string(), "hash".to_string(), 100);
        player.chips = 50;
        let threshold = 100;

        assert!(player.can_top_up(threshold));
        assert!(player.top_up(100, threshold).is_ok());
        assert_eq!(player.chips, 100);

        // Cannot top up when chips >= 100
        assert!(!player.can_top_up(threshold));
        assert!(player.top_up(100, threshold).is_err());
        assert_eq!(player.chips, 100); // Unchanged
    }

    #[test]
    fn test_chip_operations() {
        let mut player = Player::new(1, "test".to_string(), "hash".to_string(), 100);

        assert!(player.deduct_chips(50));
        assert_eq!(player.chips, 50);

        assert!(!player.deduct_chips(100)); // Insufficient chips
        assert_eq!(player.chips, 50); // Unchanged

        player.add_chips(75);
        assert_eq!(player.chips, 125);
    }
}
