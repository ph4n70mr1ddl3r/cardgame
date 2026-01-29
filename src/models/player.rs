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

    pub fn top_up(
        &mut self,
        faucet_amount: i64,
        threshold: i64,
    ) -> Result<(), crate::error::PokerError> {
        if !self.can_top_up(threshold) {
            return Err(crate::error::PokerError::Game(
                "Player already has enough chips to top up".to_string(),
            ));
        }
        self.add_chips(faucet_amount)?;
        Ok(())
    }

    pub fn deduct_chips(&mut self, amount: i64) -> Result<(), crate::error::PokerError> {
        if amount < 0 {
            return Err(crate::error::PokerError::Game(
                "Cannot deduct negative amount".to_string(),
            ));
        }
        if amount > self.chips {
            return Err(crate::error::PokerError::Game(
                "Insufficient chips".to_string(),
            ));
        }
        match self.chips.checked_sub(amount) {
            Some(new_chips) => {
                self.chips = new_chips;
                Ok(())
            }
            None => Err(crate::error::PokerError::Game(
                "Insufficient chips".to_string(),
            )),
        }
    }

    pub fn add_chips(&mut self, amount: i64) -> Result<(), crate::error::PokerError> {
        if amount < 0 {
            return Err(crate::error::PokerError::Game(
                "Cannot add negative amount".to_string(),
            ));
        }
        match self.chips.checked_add(amount) {
            Some(new_chips) => {
                self.chips = new_chips;
                Ok(())
            }
            None => Err(crate::error::PokerError::Game("Chip overflow".to_string())),
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
        assert_eq!(player.chips, 150);

        // Cannot top up when chips >= 100
        assert!(!player.can_top_up(threshold));
        assert!(player.top_up(100, threshold).is_err());
        assert_eq!(player.chips, 150);
    }

    #[test]
    fn test_chip_operations() {
        let mut player = Player::new(1, "test".to_string(), "hash".to_string(), 100);

        assert!(player.deduct_chips(50).is_ok());
        assert_eq!(player.chips, 50);

        assert!(player.deduct_chips(100).is_err());
        assert_eq!(player.chips, 50);

        assert!(player.add_chips(75).is_ok());
        assert_eq!(player.chips, 125);
    }
}
