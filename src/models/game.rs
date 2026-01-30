use super::card::{Card, Deck};
use crate::error::Result;
use serde::{Deserialize, Serialize};

/// Maximum number of players per table (currently heads-up only)
pub const MAX_PLAYERS: usize = 2;

/// Standard number of cards in a poker deck
pub const CARDS_IN_DECK: usize = 52;

/// Number of hole cards dealt to each player
pub const HOLE_CARDS: usize = 2;

/// Number of community cards (flop, turn, river)
pub const COMMUNITY_CARDS: usize = 5;

/// Minimum username length
pub const MIN_USERNAME_LEN: usize = 3;

/// Maximum username length
pub const MAX_USERNAME_LEN: usize = 20;

/// Minimum password length
pub const MIN_PASSWORD_LEN: usize = 8;

/// Maximum password length
pub const MAX_PASSWORD_LEN: usize = 128;

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

/// Represents a valid action a player can take in the current game state.
///
/// # Fields
///
/// * `action` - The type of action (Fold, Check, Call, Raise, AllIn)
/// * `min_raise` - Minimum raise amount for Raise actions, None otherwise
/// * `max_raise` - Maximum raise amount for Raise actions, None otherwise
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidAction {
    pub action: PlayerAction,
    pub min_raise: Option<i64>,
    pub max_raise: Option<i64>,
}

impl std::fmt::Display for ValidAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.action {
            PlayerAction::Raise(amount) => write!(f, "Raise to {amount}"),
            other => write!(f, "{other:?}"),
        }
    }
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
    #[must_use]
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
    pub last_raise_amount: i64,
    pub dealer_index: usize,
    pub current_player_index: Option<usize>,
    pub small_blind: i64,
    pub big_blind: i64,
    pub deck: Deck,
    pub hand_number: u64,
    pub side_pots: Vec<SidePot>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SidePot {
    pub amount: i64,
    pub eligible_players: Vec<usize>,
}

// TODO: Implement side pot calculation and distribution logic (Phase 4)
// Side pots are needed when players go all-in with different amounts
// Implementation requirements:
// - Calculate side pots when a player is all-in but others continue betting
// - Track which players are eligible for each side pot
// - Distribute side pots at showdown based on hand rankings
// - Handle multiple side pots in a single hand
// Reference: https://en.wikipedia.org/wiki/Split_pot

impl GameState {
    /// Creates a new game state for a poker table.
    ///
    /// # Arguments
    ///
    /// * `table_id` - Unique table identifier
    /// * `small_blind` - Small blind amount
    /// * `big_blind` - Big blind amount
    #[must_use]
    pub fn new(table_id: i64, small_blind: i64, big_blind: i64) -> Self {
        Self {
            table_id,
            stage: GameStage::WaitingForPlayers,
            players: Vec::new(),
            community_cards: Vec::new(),
            pot: 0,
            current_bet: 0,
            last_raise_amount: big_blind,
            dealer_index: 0,
            current_player_index: None,
            small_blind,
            big_blind,
            deck: Deck::new(),
            hand_number: 0,
            side_pots: Vec::new(),
        }
    }

    /// Adds a player to the game.
    ///
    /// First player becomes dealer, second becomes non-dealer.
    ///
    /// # Arguments
    ///
    /// * `player_id` - Player's unique identifier
    /// * `username` - Player's username
    /// * `buyin` - Amount of chips player brings to table
    ///
    /// # Errors
    ///
    /// Returns error if maximum players (2 for heads-up) already seated
    pub fn add_player(&mut self, player_id: i64, username: String, buyin: i64) -> Result<()> {
        if self.players.len() >= MAX_PLAYERS {
            return Err(crate::error::PokerError::game(format!(
                "Cannot add more than {} players in heads-up poker",
                MAX_PLAYERS
            )));
        }
        let is_dealer = self.players.is_empty();
        self.players
            .push(PlayerGameState::new(player_id, username, buyin, is_dealer));
        Ok(())
    }

    /// Checks if game can start (both players seated).
    #[must_use]
    pub fn is_ready_to_start(&self) -> bool {
        self.players.len() == MAX_PLAYERS && self.stage == GameStage::WaitingForPlayers
    }

    pub fn active_players(&self) -> impl Iterator<Item = &PlayerGameState> {
        self.players
            .iter()
            .filter(|p| !p.is_folded && !p.is_disconnected)
    }

    /// Rotates dealer button to next player.
    ///
    /// # Errors
    ///
    /// Returns error if no players are present
    pub fn next_dealer(&mut self) -> Result<()> {
        if self.players.is_empty() {
            return Err(crate::error::PokerError::game(
                "Cannot rotate dealer with no players",
            ));
        }
        self.dealer_index = (self.dealer_index + 1) % self.players.len();
        Ok(())
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
        game.add_player(1, "player1".to_string(), 100).unwrap();
        game.add_player(2, "player2".to_string(), 100).unwrap();

        assert_eq!(game.players.len(), 2);
        assert!(game.is_ready_to_start());
        assert!(game.players[0].is_dealer);
        assert!(!game.players[1].is_dealer);
    }

    #[test]
    fn test_add_too_many_players() {
        let mut game = GameState::new(1, 50, 100);
        game.add_player(1, "player1".to_string(), 100).unwrap();
        game.add_player(2, "player2".to_string(), 100).unwrap();
        assert!(game.add_player(3, "player3".to_string(), 100).is_err());
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
