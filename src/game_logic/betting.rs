use crate::error::{PokerError, Result};
use crate::models::game::{GameState, PlayerAction, PlayerGameState};

/// Betting rule validation and action application.
///
/// This struct provides methods for validating poker actions,
/// applying them to game state, and determining valid moves.
pub struct BettingRules;

impl BettingRules {
    /// Validates if a player action is legal in the current game state.
    ///
    /// This function checks all preconditions for an action including:
    /// - Player has not folded
    /// - Player is not all-in
    /// - Action-specific rules (check vs call, raise limits, etc.)
    ///
    /// # Arguments
    ///
    /// * `game` - Current game state
    /// * `player_idx` - Index of the player in the game's players vector
    /// * `action` - The action to validate
    ///
    /// # Returns
    ///
    /// * `Result<()>` - Ok if action is valid, Err with reason if invalid
    pub fn validate_action(
        game: &GameState,
        player_idx: usize,
        action: &PlayerAction,
    ) -> Result<()> {
        if game.current_player_index != Some(player_idx) {
            return Err(PokerError::invalid_action(
                "It is not this player's turn to act",
            ));
        }

        let player = &game.players[player_idx];

        if player.is_folded {
            return Err(PokerError::invalid_action("Player has folded"));
        }

        if player.is_all_in {
            return Err(PokerError::invalid_action("Player is all-in"));
        }

        match action {
            PlayerAction::Fold => Ok(()),
            PlayerAction::Check => Self::validate_check(game, player),
            PlayerAction::Call => Self::validate_call(game, player),
            PlayerAction::Raise(amount) => Self::validate_raise(game, player, *amount),
            PlayerAction::AllIn => Self::validate_all_in(player),
        }
    }

    fn validate_check(game: &GameState, player: &PlayerGameState) -> Result<()> {
        if player.bet_this_round < game.current_bet {
            return Err(PokerError::invalid_action(format!(
                "Cannot check when facing a bet of {} (have bet {})",
                game.current_bet, player.bet_this_round
            )));
        }
        Ok(())
    }

    fn validate_call(game: &GameState, player: &PlayerGameState) -> Result<()> {
        if player.bet_this_round >= game.current_bet {
            return Err(PokerError::invalid_action(
                "No bet to call (should check instead)",
            ));
        }

        let call_amount = game.current_bet - player.bet_this_round;
        if call_amount > player.chips {
            return Err(PokerError::invalid_action(format!(
                "Insufficient chips to call {} (have {})",
                call_amount, player.chips
            )));
        }

        Ok(())
    }

    fn validate_raise(game: &GameState, player: &PlayerGameState, raise_to: i64) -> Result<()> {
        if raise_to < 0 {
            return Err(PokerError::invalid_action(
                "Raise amount cannot be negative",
            ));
        }

        if raise_to == 0 {
            return Err(PokerError::invalid_action("Raise amount must be positive"));
        }

        if raise_to <= game.current_bet {
            return Err(PokerError::invalid_action(format!(
                "Raise amount {} must be greater than current bet {}",
                raise_to, game.current_bet
            )));
        }

        let total_needed = raise_to
            .checked_sub(player.bet_this_round)
            .ok_or_else(|| PokerError::invalid_action("Invalid raise calculation"))?;

        let min_raise = Self::calculate_min_raise(game)?;

        if raise_to < min_raise && total_needed < player.chips {
            return Err(PokerError::invalid_action(format!(
                "Minimum raise is {min_raise} (attempted {raise_to})"
            )));
        }

        let player_chips = player.chips;
        if total_needed > player_chips {
            return Err(PokerError::invalid_action(format!(
                "Insufficient chips for this raise (need {total_needed}, have {player_chips})"
            )));
        }

        Ok(())
    }

    fn validate_all_in(player: &PlayerGameState) -> Result<()> {
        if player.chips == 0 {
            return Err(PokerError::invalid_action(
                "Cannot go all-in when already at 0 chips",
            ));
        }
        Ok(())
    }

    /// Calculates the minimum legal raise amount based on the current game state.
    ///
    /// # Rules
    ///
    /// - If the current bet equals the big blind (first raise opportunity), minimum raise is current bet + big blind
    /// - Otherwise, minimum raise is double the current bet (standard raise rule)
    ///
    /// # Arguments
    ///
    /// * `game` - Current game state containing current bet and blind amounts
    ///
    /// # Returns
    ///
    /// * `Result<i64>` - The minimum legal raise amount
    ///
    /// # Errors
    ///
    /// Returns an error if the calculation would overflow i64
    fn calculate_min_raise(game: &GameState) -> Result<i64> {
        game.current_bet
            .checked_add(game.last_raise_amount)
            .ok_or_else(|| PokerError::game("Overflow in min raise calculation"))
    }

    /// Applies a validated action to the game state, updating chips, pot, and player status.
    ///
    /// This function modifies the game state based on the action type:
    /// - Fold: Marks player as folded
    /// - Check: No chip changes
    /// - Call: Adds chips to match current bet
    /// - Raise: Increases the current bet
    /// - AllIn: Bets all remaining chips
    ///
    /// # Arguments
    ///
    /// * `game` - Mutable reference to game state
    /// * `player_idx` - Index of the player taking the action
    /// * `action` - The action to apply
    ///
    /// # Returns
    ///
    /// * `Result<()>` - Ok if action applied successfully, Err if validation fails
    ///
    /// # Note
    ///
    /// This function calls `validate_action` internally to ensure the action is legal
    pub fn apply_action(
        game: &mut GameState,
        player_idx: usize,
        action: PlayerAction,
    ) -> Result<()> {
        if player_idx >= game.players.len() {
            return Err(PokerError::invalid_player_index(
                player_idx,
                game.players.len() - 1,
            ));
        }

        Self::validate_action(game, player_idx, &action)?;

        let player = &mut game.players[player_idx];

        match action {
            PlayerAction::Fold => {
                player.is_folded = true;
            }
            PlayerAction::Check => {
                // No chips change
            }
            PlayerAction::Call => {
                let call_amount = game.current_bet - player.bet_this_round;
                player.chips = player
                    .chips
                    .checked_sub(call_amount)
                    .ok_or_else(|| PokerError::game("Insufficient chips for call"))?;
                player.bet_this_round = player
                    .bet_this_round
                    .checked_add(call_amount)
                    .ok_or_else(|| PokerError::game("Overflow in bet calculation"))?;
                player.total_bet = player
                    .total_bet
                    .checked_add(call_amount)
                    .ok_or_else(|| PokerError::game("Overflow in total bet"))?;
                game.pot = game
                    .pot
                    .checked_add(call_amount)
                    .ok_or_else(|| PokerError::game("Overflow in pot"))?;

                if player.chips == 0 {
                    player.is_all_in = true;
                }
            }
            PlayerAction::Raise(raise_to) => {
                let amount_to_add = raise_to - player.bet_this_round;
                player.chips = player
                    .chips
                    .checked_sub(amount_to_add)
                    .ok_or_else(|| PokerError::game("Insufficient chips for raise"))?;
                player.bet_this_round = raise_to;
                player.total_bet = player
                    .total_bet
                    .checked_add(amount_to_add)
                    .ok_or_else(|| PokerError::game("Overflow in total bet"))?;
                game.pot = game
                    .pot
                    .checked_add(amount_to_add)
                    .ok_or_else(|| PokerError::game("Overflow in pot"))?;
                game.last_raise_amount = raise_to - game.current_bet;
                game.current_bet = raise_to;

                if player.chips == 0 {
                    player.is_all_in = true;
                }
            }
            PlayerAction::AllIn => {
                let all_in_amount = player.chips;
                player.chips = 0;
                player.bet_this_round = player
                    .bet_this_round
                    .checked_add(all_in_amount)
                    .ok_or_else(|| PokerError::game("Overflow in bet calculation"))?;
                player.total_bet = player
                    .total_bet
                    .checked_add(all_in_amount)
                    .ok_or_else(|| PokerError::game("Overflow in total bet"))?;
                game.pot = game
                    .pot
                    .checked_add(all_in_amount)
                    .ok_or_else(|| PokerError::game("Overflow in pot"))?;
                player.is_all_in = true;

                if player.bet_this_round > game.current_bet {
                    game.current_bet = player.bet_this_round;
                }
            }
        }

        Ok(())
    }

    /// Determines if the current betting round is complete.
    ///
    /// A round is complete when:
    /// - Only one active player remains (others folded), OR
    /// - All active players have either matched the current bet or are all-in
    ///
    /// # Arguments
    ///
    /// * `game` - Current game state
    ///
    /// # Returns
    ///
    /// * `bool` - True if the betting round is complete, false otherwise
    #[must_use]
    pub fn is_round_complete(game: &GameState) -> bool {
        let active_player_count = game.players.iter().filter(|p| !p.is_folded).count();

        if active_player_count == 1 {
            return true; // One player folded, round over
        }

        // Check if all active players have matched the current bet or are all-in
        game.players
            .iter()
            .filter(|p| !p.is_folded)
            .all(|p| p.is_all_in || p.bet_this_round == game.current_bet)
    }

    /// Get valid actions for the current player
    #[must_use = "valid actions should be used to guide player input"]
    pub fn get_valid_actions(
        game: &GameState,
        player_idx: usize,
    ) -> Result<Vec<crate::models::game::ValidAction>> {
        use crate::models::game::{PlayerAction, ValidAction};
        let player = &game.players[player_idx];
        let mut actions = Vec::new();

        if player.is_folded || player.is_all_in {
            return Ok(actions);
        }

        // Fold is always valid
        actions.push(ValidAction {
            action: PlayerAction::Fold,
            min_raise: None,
            max_raise: None,
        });

        // Check if facing a bet
        if player.bet_this_round >= game.current_bet {
            // Can check
            actions.push(ValidAction {
                action: PlayerAction::Check,
                min_raise: None,
                max_raise: None,
            });
        } else {
            // Can call
            let call_amount = game.current_bet - player.bet_this_round;
            if call_amount <= player.chips {
                actions.push(ValidAction {
                    action: PlayerAction::Call,
                    min_raise: None,
                    max_raise: None,
                });
            }
        }

        // Can always raise if have chips
        if player.chips > 0 {
            let min_raise = Self::calculate_min_raise(game)?;
            let max_raise = player
                .chips
                .checked_add(player.bet_this_round)
                .ok_or_else(|| PokerError::game("Overflow in maximum raise calculation"))?;
            actions.push(ValidAction {
                action: PlayerAction::Raise(min_raise),
                min_raise: Some(min_raise),
                max_raise: Some(max_raise),
            });

            // Can go all-in if have chips
            actions.push(ValidAction {
                action: PlayerAction::AllIn,
                min_raise: None,
                max_raise: None,
            });
        }

        Ok(actions)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game_logic::dealer::Dealer;

    #[test]
    fn test_validate_check() {
        let mut game = GameState::new(1, 50, 100);
        game.add_player(1, "player1".to_string(), 100).unwrap();
        game.add_player(2, "player2".to_string(), 100).unwrap();
        Dealer::start_new_hand(&mut game).unwrap();

        assert!(BettingRules::validate_action(&game, 0, &PlayerAction::Check).is_err());
    }

    #[test]
    fn test_validate_call() {
        let mut game = GameState::new(1, 50, 100);
        game.add_player(1, "player1".to_string(), 10000).unwrap();
        game.add_player(2, "player2".to_string(), 10000).unwrap();
        Dealer::start_new_hand(&mut game).unwrap();

        assert!(BettingRules::validate_action(&game, 1, &PlayerAction::Check).is_ok());
    }

    #[test]
    fn test_apply_call() {
        let mut game = GameState::new(1, 50, 100);
        game.add_player(1, "player1".to_string(), 10000).unwrap();
        game.add_player(2, "player2".to_string(), 10000).unwrap();
        Dealer::start_new_hand(&mut game).unwrap();

        let initial_chips = game.players[1].chips;
        BettingRules::apply_action(&mut game, 1, PlayerAction::Check).unwrap();

        assert_eq!(game.players[1].chips, initial_chips);
    }

    #[test]
    fn test_apply_raise() {
        let mut game = GameState::new(1, 50, 100);
        game.add_player(1, "player1".to_string(), 10000).unwrap();
        game.add_player(2, "player2".to_string(), 10000).unwrap();
        Dealer::start_new_hand(&mut game).unwrap();

        BettingRules::apply_action(&mut game, 1, PlayerAction::Raise(200)).unwrap();

        assert_eq!(game.current_bet, 200);
        assert_eq!(game.players[1].bet_this_round, 200);
    }

    #[test]
    fn test_apply_fold() {
        let mut game = GameState::new(1, 50, 100);
        game.add_player(1, "player1".to_string(), 10000).unwrap();
        game.add_player(2, "player2".to_string(), 10000).unwrap();
        Dealer::start_new_hand(&mut game).unwrap();

        BettingRules::apply_action(&mut game, 1, PlayerAction::Fold).unwrap();

        assert!(game.players[1].is_folded);
    }

    #[test]
    fn test_apply_all_in() {
        let mut game = GameState::new(1, 50, 100);
        game.add_player(1, "player1".to_string(), 5000).unwrap();
        game.add_player(2, "player2".to_string(), 5000).unwrap();
        Dealer::start_new_hand(&mut game).unwrap();

        BettingRules::apply_action(&mut game, 1, PlayerAction::AllIn).unwrap();

        assert_eq!(game.players[1].chips, 0);
        assert!(game.players[1].is_all_in);
    }

    #[test]
    fn test_round_complete_one_fold() {
        let mut game = GameState::new(1, 50, 100);
        game.add_player(1, "player1".to_string(), 100).unwrap();
        game.add_player(2, "player2".to_string(), 100).unwrap();
        Dealer::start_new_hand(&mut game).unwrap();

        game.players[0].is_folded = true;

        assert!(BettingRules::is_round_complete(&game));
    }

    #[test]
    fn test_round_complete_bets_matched() {
        let mut game = GameState::new(1, 50, 100);
        game.add_player(1, "player1".to_string(), 10000).unwrap();
        game.add_player(2, "player2".to_string(), 10000).unwrap();
        Dealer::start_new_hand(&mut game).unwrap();

        game.players[0].bet_this_round = game.big_blind;
        game.players[1].bet_this_round = game.big_blind;

        assert!(BettingRules::is_round_complete(&game));
    }

    #[test]
    fn test_get_valid_actions() {
        let mut game = GameState::new(1, 50, 100);
        game.add_player(1, "player1".to_string(), 10000).unwrap();
        game.add_player(2, "player2".to_string(), 10000).unwrap();
        Dealer::start_new_hand(&mut game).unwrap();

        let actions = BettingRules::get_valid_actions(&game, 1).unwrap();

        assert!(actions
            .iter()
            .any(|a| matches!(a.action, PlayerAction::Fold)));
        assert!(actions
            .iter()
            .any(|a| matches!(a.action, PlayerAction::Check)));
        assert!(actions
            .iter()
            .any(|a| matches!(a.action, PlayerAction::Raise(_))));
        assert!(actions
            .iter()
            .any(|a| matches!(a.action, PlayerAction::AllIn)));
    }

    #[test]
    fn test_min_raise_calculation() {
        let mut game = GameState::new(1, 50, 100);
        game.add_player(1, "player1".to_string(), 10000).unwrap();
        game.add_player(2, "player2".to_string(), 10000).unwrap();
        Dealer::start_new_hand(&mut game).unwrap();

        let min_raise = BettingRules::calculate_min_raise(&game).unwrap();
        assert_eq!(min_raise, 200);

        BettingRules::apply_action(&mut game, 1, PlayerAction::Raise(200)).unwrap();

        let min_raise = BettingRules::calculate_min_raise(&game).unwrap();
        assert_eq!(min_raise, 300);
    }

    #[test]
    fn test_last_raise_amount_tracking() {
        let mut game = GameState::new(1, 50, 100);
        game.add_player(1, "player1".to_string(), 10000).unwrap();
        game.add_player(2, "player2".to_string(), 10000).unwrap();
        Dealer::start_new_hand(&mut game).unwrap();

        assert_eq!(game.last_raise_amount, 100);

        BettingRules::apply_action(&mut game, 1, PlayerAction::Raise(200)).unwrap();
        assert_eq!(game.last_raise_amount, 100);

        game.current_player_index = Some(0);
        BettingRules::apply_action(&mut game, 0, PlayerAction::Raise(400)).unwrap();
        assert_eq!(game.last_raise_amount, 200);
    }

    #[test]
    fn test_invalid_raise_amounts() {
        let mut game = GameState::new(1, 50, 100);
        game.add_player(1, "player1".to_string(), 10000).unwrap();
        game.add_player(2, "player2".to_string(), 10000).unwrap();
        Dealer::start_new_hand(&mut game).unwrap();

        assert!(BettingRules::validate_action(&game, 1, &PlayerAction::Raise(0)).is_err());
        assert!(BettingRules::validate_action(&game, 1, &PlayerAction::Raise(-50)).is_err());
        assert!(BettingRules::validate_action(&game, 1, &PlayerAction::Raise(99)).is_err());
    }

    #[test]
    fn test_action_after_fold() {
        let mut game = GameState::new(1, 50, 100);
        game.add_player(1, "player1".to_string(), 10000).unwrap();
        game.add_player(2, "player2".to_string(), 10000).unwrap();
        Dealer::start_new_hand(&mut game).unwrap();

        game.players[0].is_folded = true;
        assert!(BettingRules::validate_action(&game, 0, &PlayerAction::Check).is_err());
        assert!(BettingRules::validate_action(&game, 0, &PlayerAction::Call).is_err());
    }
}
