use super::card::{Card, Deck};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GameStage {
    WaitingForPlayers,
    PreFlop,
    Flop,
    Turn,
    River,
    Showdown,
    Finished,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlayerAction {
    Fold,
    Check,
    Call,
    Raise(i64),
    AllIn,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerGameState {
    pub player_id: i64,
    pub username: String,
    pub chips: i64,
    pub bet_this_round: i64,
    pub total_bet: i64,
    pub hole_cards: Vec<Card>,
    pub is_folded: bool,
    pub is_all_in: bool,
    pub is_dealer: bool,
    pub is_small_blind: bool,
    pub is_big_blind: bool,
    pub is_disconnected: bool,
}

impl PlayerGameState {
    pub fn new(player_id: i64, username: String, chips: i64, is_dealer: bool) -> Self {
        Self {
            player_id,
            username,
            chips,
            bet_this_round: 0,
            total_bet: 0,
            hole_cards: Vec::new(),
            is_folded: false,
            is_all_in: false,
            is_dealer,
            is_small_blind: !is_dealer, // Heads-up: non-dealer is SB
            is_big_blind: is_dealer,    // Heads-up: dealer is BB
            is_disconnected: false,
        }
    }

    pub fn reset_for_new_hand(&mut self, is_dealer: bool) {
        self.bet_this_round = 0;
        self.total_bet = 0;
        self.hole_cards.clear();
        self.is_folded = false;
        self.is_all_in = false;
        self.is_dealer = is_dealer;
        self.is_small_blind = !is_dealer;
        self.is_big_blind = is_dealer;
    }

    pub fn reset_round_bet(&mut self) {
        self.bet_this_round = 0;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    pub table_id: i64,
    pub stage: GameStage,
    pub players: Vec<PlayerGameState>,
    pub community_cards: Vec<Card>,
    pub pot: i64,
    pub current_bet: i64,
    pub dealer_index: usize,
    pub current_player_index: Option<usize>,
    pub small_blind: i64,
    pub big_blind: i64,
    pub deck: Deck,
    pub hand_number: u64,
}

impl GameState {
    pub fn new(table_id: i64, small_blind: i64, big_blind: i64) -> Self {
        Self {
            table_id,
            stage: GameStage::WaitingForPlayers,
            players: Vec::new(),
            community_cards: Vec::new(),
            pot: 0,
            current_bet: 0,
            dealer_index: 0,
            current_player_index: None,
            small_blind,
            big_blind,
            deck: Deck::new(),
            hand_number: 0,
        }
    }

    pub fn add_player(&mut self, player_id: i64, username: String, buyin: i64) {
        if self.players.len() >= 2 {
            panic!("Cannot add more than 2 players in heads-up poker");
        }
        let is_dealer = self.players.is_empty();
        self.players
            .push(PlayerGameState::new(player_id, username, buyin, is_dealer));
    }

    pub fn is_ready_to_start(&self) -> bool {
        self.players.len() == 2 && self.stage == GameStage::WaitingForPlayers
    }

    pub fn active_players(&self) -> Vec<&PlayerGameState> {
        self.players
            .iter()
            .filter(|p| !p.is_folded && !p.is_disconnected)
            .collect()
    }

    pub fn next_dealer(&mut self) {
        self.dealer_index = (self.dealer_index + 1) % self.players.len();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_state_creation() {
        let game = GameState::new(1, 50, 100);
        assert_eq!(game.stage, GameStage::WaitingForPlayers);
        assert_eq!(game.small_blind, 50);
        assert_eq!(game.big_blind, 100);
        assert_eq!(game.pot, 0);
    }

    #[test]
    fn test_add_players() {
        let mut game = GameState::new(1, 50, 100);
        game.add_player(1, "player1".to_string(), 100);
        game.add_player(2, "player2".to_string(), 100);

        assert_eq!(game.players.len(), 2);
        assert!(game.is_ready_to_start());
        assert!(game.players[0].is_dealer);
        assert!(!game.players[1].is_dealer);
    }

    #[test]
    fn test_player_game_state_reset() {
        let mut player = PlayerGameState::new(1, "test".to_string(), 100, true);
        player.total_bet = 50;
        player.is_folded = true;

        player.reset_for_new_hand(false);
        assert_eq!(player.total_bet, 0);
        assert!(!player.is_folded);
        assert!(!player.is_dealer);
    }
}
