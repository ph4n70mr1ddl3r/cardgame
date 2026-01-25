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
        game.stage = GameStage::PreFlop;
        game.side_pots.clear();

        // Reset player states for new hand
        for (i, player) in game.players.iter_mut().enumerate() {
            let is_dealer = i == game.dealer_index;
            player.reset_for_new_hand(is_dealer);
        }

        // Post blinds
        Self::post_blinds(game)?;

        // Deal hole cards (2 cards to each player)
        Self::deal_hole_cards(game)?;

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

        let sb_amount = game.small_blind.min(game.players[sb_player_idx].chips);
        game.players[sb_player_idx].chips = game.players[sb_player_idx]
            .chips
            .checked_sub(sb_amount)
            .ok_or_else(|| {
                crate::error::PokerError::Game("Chip underflow posting small blind".to_string())
            })?;
        game.players[sb_player_idx].bet_this_round = sb_amount;
        game.players[sb_player_idx].total_bet = sb_amount;
        game.pot = game.pot.checked_add(sb_amount).ok_or_else(|| {
            crate::error::PokerError::Game("Pot overflow posting small blind".to_string())
        })?;

        let bb_amount = game.big_blind.min(game.players[bb_player_idx].chips);
        game.players[bb_player_idx].chips = game.players[bb_player_idx]
            .chips
            .checked_sub(bb_amount)
            .ok_or_else(|| {
                crate::error::PokerError::Game("Chip underflow posting big blind".to_string())
            })?;
        game.players[bb_player_idx].bet_this_round = bb_amount;
        game.players[bb_player_idx].total_bet = bb_amount;
        game.pot = game.pot.checked_add(bb_amount).ok_or_else(|| {
            crate::error::PokerError::Game("Pot overflow posting big blind".to_string())
        })?;
        game.current_bet = bb_amount;

        if game.players[sb_player_idx].chips == 0 {
            game.players[sb_player_idx].is_all_in = true;
        }
        if game.players[bb_player_idx].chips == 0 {
            game.players[bb_player_idx].is_all_in = true;
        }

        Ok(())
    }

    fn deal_hole_cards(game: &mut GameState) -> Result<()> {
        for _ in 0..2 {
            for player in &mut game.players {
                if let Some(card) = game.deck.deal() {
                    player.hole_cards.push(card);
                }
            }
        }
        Ok(())
    }

    pub fn deal_flop(game: &mut GameState) -> Result<()> {
        game.stage = GameStage::Flop;
        game.current_bet = 0;

        // Burn one card
        game.deck.deal();

        // Deal 3 community cards
        for _ in 0..3 {
            if let Some(card) = game.deck.deal() {
                game.community_cards.push(card);
            }
        }

        // Reset round bets
        for player in &mut game.players {
            player.reset_round_bet();
        }

        game.current_player_index = Some((game.dealer_index + 1) % game.players.len());

        Ok(())
    }

    pub fn deal_turn(game: &mut GameState) -> Result<()> {
        game.stage = GameStage::Turn;
        game.current_bet = 0;

        // Burn one card
        game.deck.deal();

        // Deal 1 community card
        if let Some(card) = game.deck.deal() {
            game.community_cards.push(card);
        }

        // Reset round bets
        for player in &mut game.players {
            player.reset_round_bet();
        }

        game.current_player_index = Some((game.dealer_index + 1) % game.players.len());

        Ok(())
    }

    pub fn deal_river(game: &mut GameState) -> Result<()> {
        game.stage = GameStage::River;
        game.current_bet = 0;

        // Burn one card
        game.deck.deal();

        // Deal 1 community card
        if let Some(card) = game.deck.deal() {
            game.community_cards.push(card);
        }

        // Reset round bets
        for player in &mut game.players {
            player.reset_round_bet();
        }

        game.current_player_index = Some((game.dealer_index + 1) % game.players.len());

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
