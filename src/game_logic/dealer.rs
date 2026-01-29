use crate::error::Result;
use crate::models::card::Deck;
use crate::models::game::{GameStage, GameState};

pub struct Dealer;

impl Dealer {
    pub fn start_new_hand(game: &mut GameState) -> Result<()> {
        // Reset for new hand
        game.hand_number += 1;
        game.deck = Deck::new();
        game.deck.shuffle();
        game.community_cards.clear();
        game.pot = 0;
        game.current_bet = 0;
        game.last_raise_amount = game.big_blind;
        game.stage = GameStage::PreFlop;
        game.side_pots.clear();

        // Reset player states for new hand
        for (i, player) in game.players.iter_mut().enumerate() {
            let is_dealer = i == game.dealer_index;
            player.reset_for_new_hand(is_dealer);
        }

        // Post blinds
        Self::post_blinds(game)?;

        Self::deal_hole_cards(game);

        // Set current player (small blind acts first preflop in heads-up)
        game.current_player_index = Some((game.dealer_index + 1) % game.players.len());

        Ok(())
    }

    fn post_blinds(game: &mut GameState) -> Result<()> {
        if game.players.len() < 2 {
            return Err(crate::error::PokerError::Game(
                "Need at least 2 players to post blinds".to_string(),
            ));
        }

        let sb_player_idx = game.dealer_index;
        let bb_player_idx = (game.dealer_index + 1) % game.players.len();

        let sb_actual = game.small_blind.min(game.players[sb_player_idx].chips);
        let bb_actual = game.big_blind.min(game.players[bb_player_idx].chips);

        Self::post_blind(game, sb_player_idx, game.small_blind)?;
        Self::post_blind(game, bb_player_idx, game.big_blind)?;
        game.current_bet = bb_actual.max(sb_actual);

        Ok(())
    }

    fn post_blind(game: &mut GameState, player_idx: usize, blind_amount: i64) -> Result<()> {
        let actual_amount = blind_amount.min(game.players[player_idx].chips);
        game.players[player_idx].chips = game.players[player_idx]
            .chips
            .checked_sub(actual_amount)
            .ok_or_else(|| {
                crate::error::PokerError::Game("Chip underflow posting blind".to_string())
            })?;
        game.players[player_idx].bet_this_round = actual_amount;
        game.players[player_idx].total_bet = actual_amount;
        game.pot = game.pot.checked_add(actual_amount).ok_or_else(|| {
            crate::error::PokerError::Game("Pot overflow posting blind".to_string())
        })?;

        if game.players[player_idx].chips == 0 {
            game.players[player_idx].is_all_in = true;
        }

        Ok(())
    }

    fn deal_hole_cards(game: &mut GameState) {
        for _ in 0..crate::models::game::HOLE_CARDS {
            for player in &mut game.players {
                if let Some(card) = game.deck.deal() {
                    player.hole_cards.push(card);
                }
            }
        }
    }

    fn deal_community_cards(game: &mut GameState, stage: GameStage, num_cards: usize) {
        game.stage = stage;
        game.current_bet = 0;
        game.last_raise_amount = game.big_blind;

        // Burn one card: In poker, the top card of the deck is discarded ("burned")
        // before dealing community cards to prevent marking or card counting.
        // The burned card is not used in play.
        if game.deck.deal().is_some() {
            tracing::debug!("Burned card before dealing {:?}", stage);
        }

        // Deal community cards to the table
        for _ in 0..num_cards {
            if let Some(card) = game.deck.deal() {
                game.community_cards.push(card);
            }
        }

        // Reset round bets for the new betting round
        for player in &mut game.players {
            player.reset_round_bet();
        }

        // Set current player to small blind (non-dealer in heads-up)
        game.current_player_index = Some((game.dealer_index + 1) % game.players.len());
    }

    pub fn deal_flop(game: &mut GameState) -> Result<()> {
        Self::deal_community_cards(game, GameStage::Flop, 3);
        Ok(())
    }

    pub fn deal_turn(game: &mut GameState) -> Result<()> {
        Self::deal_community_cards(game, GameStage::Turn, 1);
        Ok(())
    }

    pub fn deal_river(game: &mut GameState) -> Result<()> {
        Self::deal_community_cards(game, GameStage::River, 1);
        Ok(())
    }

    pub fn advance_to_showdown(game: &mut GameState) {
        game.stage = GameStage::Showdown;
        game.current_player_index = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_start_new_hand() {
        let mut game = GameState::new(1, 50, 100);
        game.add_player(1, "player1".to_string(), 100).unwrap();
        game.add_player(2, "player2".to_string(), 100).unwrap();

        Dealer::start_new_hand(&mut game).unwrap();

        assert_eq!(game.stage, GameStage::PreFlop);
        assert_eq!(game.hand_number, 1);
        assert_eq!(game.players[0].hole_cards.len(), 2);
        assert_eq!(game.players[1].hole_cards.len(), 2);
        assert!(game.pot > 0);
    }

    #[test]
    fn test_blinds_posted() {
        let mut game = GameState::new(1, 50, 100);
        game.add_player(1, "player1".to_string(), 10000).unwrap();
        game.add_player(2, "player2".to_string(), 10000).unwrap();

        Dealer::start_new_hand(&mut game).unwrap();

        assert_eq!(game.pot, 150);
    }

    #[test]
    fn test_deal_flop() {
        let mut game = GameState::new(1, 50, 100);
        game.add_player(1, "player1".to_string(), 100).unwrap();
        game.add_player(2, "player2".to_string(), 100).unwrap();

        Dealer::start_new_hand(&mut game).unwrap();
        Dealer::deal_flop(&mut game).unwrap();

        assert_eq!(game.stage, GameStage::Flop);
        assert_eq!(game.community_cards.len(), 3);
    }

    #[test]
    fn test_deal_turn() {
        let mut game = GameState::new(1, 50, 100);
        game.add_player(1, "player1".to_string(), 100).unwrap();
        game.add_player(2, "player2".to_string(), 100).unwrap();

        Dealer::start_new_hand(&mut game).unwrap();
        Dealer::deal_flop(&mut game).unwrap();
        Dealer::deal_turn(&mut game).unwrap();

        assert_eq!(game.stage, GameStage::Turn);
        assert_eq!(game.community_cards.len(), 4);
    }

    #[test]
    fn test_deal_river() {
        let mut game = GameState::new(1, 50, 100);
        game.add_player(1, "player1".to_string(), 100).unwrap();
        game.add_player(2, "player2".to_string(), 100).unwrap();

        Dealer::start_new_hand(&mut game).unwrap();
        Dealer::deal_flop(&mut game).unwrap();
        Dealer::deal_turn(&mut game).unwrap();
        Dealer::deal_river(&mut game).unwrap();

        assert_eq!(game.stage, GameStage::River);
        assert_eq!(game.community_cards.len(), 5);
    }
}
