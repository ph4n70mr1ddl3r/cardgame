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
        let call_amount = game.current_bet - player.bet_this_round;
        let total_needed = raise_to - player.bet_this_round;
        
        if raise_to <= game.current_bet {
            return Err(PokerError::InvalidAction(
                "Raise amount must be greater than current bet".to_string(),
            ));
        }
        
        // Minimum raise is 2x the big blind or 2x the last raise
        let min_raise = if game.current_bet == game.big_blind {
            game.current_bet + game.big_blind
        } else {
            game.current_bet * 2
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
            return Err(PokerError::InvalidAction("No chips to go all-in with".to_string()));
        }
        Ok(())
    }
    
    /// Apply an action to the game state
    pub fn apply_action(
        game: &mut GameState,
        player_idx: usize,
        action: PlayerAction,
    ) -> Result<()> {
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
                player.chips -= call_amount;
                player.bet_this_round += call_amount;
                player.total_bet += call_amount;
                game.pot += call_amount;
                
                if player.chips == 0 {
                    player.is_all_in = true;
                }
            }
            PlayerAction::Raise(raise_to) => {
                let amount_to_add = raise_to - player.bet_this_round;
                player.chips -= amount_to_add;
                player.bet_this_round = raise_to;
                player.total_bet += amount_to_add;
                game.pot += amount_to_add;
                game.current_bet = raise_to;
                
                if player.chips == 0 {
                    player.is_all_in = true;
                }
            }
            PlayerAction::AllIn => {
                let all_in_amount = player.chips;
                player.chips = 0;
                player.bet_this_round += all_in_amount;
                player.total_bet += all_in_amount;
                game.pot += all_in_amount;
                player.is_all_in = true;
                
                // Update current bet if this all-in is a raise
                if player.bet_this_round > game.current_bet {
                    game.current_bet = player.bet_this_round;
                }
            }
        }
        
        Ok(())
    }
    
    /// Check if the betting round is complete
    pub fn is_round_complete(game: &GameState) -> bool {
        let active_players: Vec<&PlayerGameState> = game
            .players
            .iter()
            .filter(|p| !p.is_folded)
            .collect();
        
        if active_players.len() == 1 {
            return true; // One player folded, round over
        }
        
        // Check if all active players have matched the current bet or are all-in
        active_players.iter().all(|p| {
            p.is_all_in || p.bet_this_round == game.current_bet
        })
    }
    
    /// Get valid actions for the current player
    pub fn get_valid_actions(game: &GameState, player_idx: usize) -> Vec<PlayerAction> {
        let player = &game.players[player_idx];
        let mut actions = Vec::new();
        
        if player.is_folded || player.is_all_in {
            return actions;
        }
        
        // Fold is always valid
        actions.push(PlayerAction::Fold);
        
        // Check if facing a bet
        if player.bet_this_round >= game.current_bet {
            // Can check
            actions.push(PlayerAction::Check);
        } else {
            // Can call
            let call_amount = game.current_bet - player.bet_this_round;
            if call_amount <= player.chips {
                actions.push(PlayerAction::Call);
            }
        }
        
        // Can always raise if have chips
        if player.chips > 0 {
            actions.push(PlayerAction::Raise(0)); // Placeholder value
        }
        
        // Can always go all-in if have chips
        if player.chips > 0 {
            actions.push(PlayerAction::AllIn);
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
        let mut game = GameState::new(1, 0.5, 1.0);
        game.add_player(1, "player1".to_string(), 100);
        game.add_player(2, "player2".to_string(), 100);
        Dealer::start_new_hand(&mut game).unwrap();
        
        // Player facing BB cannot check
        assert!(BettingRules::validate_action(&game, 0, &PlayerAction::Check).is_err());
    }

    #[test]
    fn test_validate_call() {
        let mut game = GameState::new(1, 0.5, 1.0);
        game.add_player(1, "player1".to_string(), 100);
        game.add_player(2, "player2".to_string(), 100);
        Dealer::start_new_hand(&mut game).unwrap();
        
        // Small blind can call the BB
        assert!(BettingRules::validate_action(&game, 0, &PlayerAction::Call).is_ok());
    }

    #[test]
    fn test_apply_call() {
        let mut game = GameState::new(1, 0.5, 1.0);
        game.add_player(1, "player1".to_string(), 100);
        game.add_player(2, "player2".to_string(), 100);
        Dealer::start_new_hand(&mut game).unwrap();
        
        let initial_chips = game.players[0].chips;
        BettingRules::apply_action(&mut game, 0, PlayerAction::Call).unwrap();
        
        // Should have called the difference (BB - SB)
        assert!(game.players[0].chips < initial_chips);
    }

    #[test]
    fn test_apply_raise() {
        let mut game = GameState::new(1, 0.5, 1.0);
        game.add_player(1, "player1".to_string(), 100);
        game.add_player(2, "player2".to_string(), 100);
        Dealer::start_new_hand(&mut game).unwrap();
        
        // Raise to 3 (min raise rule)
        BettingRules::apply_action(&mut game, 0, PlayerAction::Raise(3)).unwrap();
        
        assert_eq!(game.current_bet, 3);
        assert_eq!(game.players[0].bet_this_round, 3);
    }

    #[test]
    fn test_apply_fold() {
        let mut game = GameState::new(1, 0.5, 1.0);
        game.add_player(1, "player1".to_string(), 100);
        game.add_player(2, "player2".to_string(), 100);
        Dealer::start_new_hand(&mut game).unwrap();
        
        BettingRules::apply_action(&mut game, 0, PlayerAction::Fold).unwrap();
        
        assert!(game.players[0].is_folded);
    }

    #[test]
    fn test_apply_all_in() {
        let mut game = GameState::new(1, 0.5, 1.0);
        game.add_player(1, "player1".to_string(), 50);
        game.add_player(2, "player2".to_string(), 100);
        Dealer::start_new_hand(&mut game).unwrap();
        
        BettingRules::apply_action(&mut game, 0, PlayerAction::AllIn).unwrap();
        
        assert_eq!(game.players[0].chips, 0);
        assert!(game.players[0].is_all_in);
    }

    #[test]
    fn test_round_complete_one_fold() {
        let mut game = GameState::new(1, 0.5, 1.0);
        game.add_player(1, "player1".to_string(), 100);
        game.add_player(2, "player2".to_string(), 100);
        Dealer::start_new_hand(&mut game).unwrap();
        
        game.players[0].is_folded = true;
        
        assert!(BettingRules::is_round_complete(&game));
    }

    #[test]
    fn test_round_complete_bets_matched() {
        let mut game = GameState::new(1, 0.5, 1.0);
        game.add_player(1, "player1".to_string(), 100);
        game.add_player(2, "player2".to_string(), 100);
        Dealer::start_new_hand(&mut game).unwrap();
        
        // Both players match BB
        game.players[0].bet_this_round = game.big_blind;
        game.players[1].bet_this_round = game.big_blind;
        
        assert!(BettingRules::is_round_complete(&game));
    }

    #[test]
    fn test_get_valid_actions() {
        let mut game = GameState::new(1, 0.5, 1.0);
        game.add_player(1, "player1".to_string(), 100);
        game.add_player(2, "player2".to_string(), 100);
        Dealer::start_new_hand(&mut game).unwrap();
        
        let actions = BettingRules::get_valid_actions(&game, 0);
        
        // Facing a bet: should have Fold, Call, Raise, AllIn
        assert!(actions.contains(&PlayerAction::Fold));
        assert!(actions.contains(&PlayerAction::Call));
        assert!(actions.iter().any(|a| matches!(a, PlayerAction::Raise(_))));
        assert!(actions.contains(&PlayerAction::AllIn));
    }
}
