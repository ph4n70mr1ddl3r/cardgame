use crate::error::{PokerError, Result};
use crate::models::game::{GameState, PlayerAction, PlayerGameState};

pub struct BettingRules;

impl BettingRules {
    /// Validate if a player action is legal in the current game state
    pub fn validate_action(
        game: &GameState,
        player_idx: usize,
        action: &PlayerAction,
    ) -> Result<()> {
        let player = &game.players[player_idx];

        if player.is_folded {
            return Err(PokerError::InvalidAction("Player has folded".to_string()));
        }

        if player.is_all_in {
            return Err(PokerError::InvalidAction("Player is all-in".to_string()));
        }

        match action {
            PlayerAction::Fold => Ok(()),
            PlayerAction::Check => Self::validate_check(game, player),
            PlayerAction::Call => Self::validate_call(game, player),
            PlayerAction::Raise(amount) => Self::validate_raise(game, player, *amount),
            PlayerAction::AllIn => Self::validate_all_in(game, player),
        }
    }

    fn validate_check(game: &GameState, player: &PlayerGameState) -> Result<()> {
        if player.bet_this_round < game.current_bet {
            return Err(PokerError::InvalidAction(
                "Cannot check when facing a bet".to_string(),
            ));
        }
        Ok(())
    }

    fn validate_call(game: &GameState, player: &PlayerGameState) -> Result<()> {
        if player.bet_this_round >= game.current_bet {
            return Err(PokerError::InvalidAction(
                "No bet to call (should check instead)".to_string(),
            ));
        }

        let call_amount = game.current_bet - player.bet_this_round;
        if call_amount > player.chips {
            return Err(PokerError::InvalidAction(
                "Insufficient chips to call (go all-in instead)".to_string(),
            ));
        }

        Ok(())
    }

    fn validate_raise(game: &GameState, player: &PlayerGameState, raise_to: i64) -> Result<()> {
        if raise_to <= 0 {
            return Err(PokerError::InvalidAction(
                "Raise amount must be positive".to_string(),
            ));
        }

        let total_needed = raise_to
            .checked_sub(player.bet_this_round)
            .ok_or_else(|| PokerError::InvalidAction("Invalid raise calculation".to_string()))?;

        if raise_to <= game.current_bet {
            return Err(PokerError::InvalidAction(
                "Raise amount must be greater than current bet".to_string(),
            ));
        }

        let min_raise = if game.current_bet == game.big_blind {
            game.current_bet
                .checked_add(game.big_blind)
                .ok_or_else(|| PokerError::Game("Overflow in min raise calculation".to_string()))?
        } else {
            game.current_bet
                .checked_mul(2)
                .ok_or_else(|| PokerError::Game("Overflow in min raise calculation".to_string()))?
        };

        if raise_to < min_raise && total_needed < player.chips {
            return Err(PokerError::InvalidAction(format!(
                "Minimum raise is {}",
                min_raise
            )));
        }

        if total_needed > player.chips {
            return Err(PokerError::InvalidAction(
                "Insufficient chips for this raise (go all-in instead)".to_string(),
            ));
        }

        Ok(())
    }

    fn validate_all_in(_game: &GameState, player: &PlayerGameState) -> Result<()> {
        if player.chips == 0 {
            return Err(PokerError::InvalidAction(
                "No chips to go all-in with".to_string(),
            ));
        }
        Ok(())
    }

    /// Apply an action to the game state
    pub fn apply_action(
        game: &mut GameState,
        player_idx: usize,
        action: PlayerAction,
    ) -> Result<()> {
        if player_idx >= game.players.len() {
            return Err(PokerError::Game("Invalid player index".to_string()));
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
                    .ok_or_else(|| PokerError::Game("Insufficient chips for call".to_string()))?;
                player.bet_this_round = player
                    .bet_this_round
                    .checked_add(call_amount)
                    .ok_or_else(|| PokerError::Game("Overflow in bet calculation".to_string()))?;
                player.total_bet = player
                    .total_bet
                    .checked_add(call_amount)
                    .ok_or_else(|| PokerError::Game("Overflow in total bet".to_string()))?;
                game.pot = game
                    .pot
                    .checked_add(call_amount)
                    .ok_or_else(|| PokerError::Game("Overflow in pot".to_string()))?;

                if player.chips == 0 {
                    player.is_all_in = true;
                }
            }
            PlayerAction::Raise(raise_to) => {
                let amount_to_add = raise_to - player.bet_this_round;
                player.chips = player
                    .chips
                    .checked_sub(amount_to_add)
                    .ok_or_else(|| PokerError::Game("Insufficient chips for raise".to_string()))?;
                player.bet_this_round = raise_to;
                player.total_bet = player
                    .total_bet
                    .checked_add(amount_to_add)
                    .ok_or_else(|| PokerError::Game("Overflow in total bet".to_string()))?;
                game.pot = game
                    .pot
                    .checked_add(amount_to_add)
                    .ok_or_else(|| PokerError::Game("Overflow in pot".to_string()))?;
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
                    .ok_or_else(|| PokerError::Game("Overflow in bet calculation".to_string()))?;
                player.total_bet = player
                    .total_bet
                    .checked_add(all_in_amount)
                    .ok_or_else(|| PokerError::Game("Overflow in total bet".to_string()))?;
                game.pot = game
                    .pot
                    .checked_add(all_in_amount)
                    .ok_or_else(|| PokerError::Game("Overflow in pot".to_string()))?;
                player.is_all_in = true;

                if player.bet_this_round > game.current_bet {
                    game.current_bet = player.bet_this_round;
                }
            }
        }

        Ok(())
    }

    /// Check if the betting round is complete
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
    pub fn get_valid_actions(
        game: &GameState,
        player_idx: usize,
    ) -> Vec<crate::models::game::ValidAction> {
        use crate::models::game::{PlayerAction, ValidAction};
        let player = &game.players[player_idx];
        let mut actions = Vec::new();

        if player.is_folded || player.is_all_in {
            return actions;
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
            let min_raise = if game.current_bet == game.big_blind {
                game.current_bet
                    .checked_add(game.big_blind)
                    .unwrap_or(i64::MAX)
            } else {
                game.current_bet.checked_mul(2).unwrap_or(i64::MAX)
            };
            let max_raise = player
                .chips
                .checked_add(player.bet_this_round)
                .unwrap_or(i64::MAX);
            actions.push(ValidAction {
                action: PlayerAction::Raise(0),
                min_raise: Some(min_raise),
                max_raise: Some(max_raise),
            });
        }

        // Can always go all-in if have chips
        if player.chips > 0 {
            actions.push(ValidAction {
                action: PlayerAction::AllIn,
                min_raise: None,
                max_raise: None,
            });
        }

        actions
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
        game.add_player(1, "player1".to_string(), 100).unwrap();
        game.add_player(2, "player2".to_string(), 100).unwrap();
        Dealer::start_new_hand(&mut game).unwrap();

        assert!(BettingRules::validate_action(&game, 0, &PlayerAction::Call).is_ok());
    }

    #[test]
    fn test_apply_call() {
        let mut game = GameState::new(1, 50, 100);
        game.add_player(1, "player1".to_string(), 100).unwrap();
        game.add_player(2, "player2".to_string(), 100).unwrap();
        Dealer::start_new_hand(&mut game).unwrap();

        let initial_chips = game.players[0].chips;
        BettingRules::apply_action(&mut game, 0, PlayerAction::Call).unwrap();

        assert!(game.players[0].chips < initial_chips);
    }

    #[test]
    fn test_apply_raise() {
        let mut game = GameState::new(1, 50, 100);
        game.add_player(1, "player1".to_string(), 10000).unwrap();
        game.add_player(2, "player2".to_string(), 10000).unwrap();
        Dealer::start_new_hand(&mut game).unwrap();

        BettingRules::apply_action(&mut game, 0, PlayerAction::Raise(200)).unwrap();

        assert_eq!(game.current_bet, 200);
        assert_eq!(game.players[0].bet_this_round, 200);
    }

    #[test]
    fn test_apply_fold() {
        let mut game = GameState::new(1, 50, 100);
        game.add_player(1, "player1".to_string(), 100).unwrap();
        game.add_player(2, "player2".to_string(), 100).unwrap();
        Dealer::start_new_hand(&mut game).unwrap();

        BettingRules::apply_action(&mut game, 0, PlayerAction::Fold).unwrap();

        assert!(game.players[0].is_folded);
    }

    #[test]
    fn test_apply_all_in() {
        let mut game = GameState::new(1, 50, 100);
        game.add_player(1, "player1".to_string(), 5000).unwrap();
        game.add_player(2, "player2".to_string(), 5000).unwrap();
        Dealer::start_new_hand(&mut game).unwrap();

        BettingRules::apply_action(&mut game, 0, PlayerAction::AllIn).unwrap();

        assert_eq!(game.players[0].chips, 0);
        assert!(game.players[0].is_all_in);
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
        game.add_player(1, "player1".to_string(), 100).unwrap();
        game.add_player(2, "player2".to_string(), 100).unwrap();
        Dealer::start_new_hand(&mut game).unwrap();

        let actions = BettingRules::get_valid_actions(&game, 0);

        assert!(actions
            .iter()
            .any(|a| matches!(a.action, PlayerAction::Fold)));
        assert!(actions
            .iter()
            .any(|a| matches!(a.action, PlayerAction::Call)));
        assert!(actions
            .iter()
            .any(|a| matches!(a.action, PlayerAction::Raise(_))));
        assert!(actions
            .iter()
            .any(|a| matches!(a.action, PlayerAction::AllIn)));
    }
}
