use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum Rank {
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
    Ace = 14,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

impl Card {
    #[must_use]
    pub fn new(suit: Suit, rank: Rank) -> Self {
        Self { suit, rank }
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let rank_str = match self.rank {
            Rank::Two => "2",
            Rank::Three => "3",
            Rank::Four => "4",
            Rank::Five => "5",
            Rank::Six => "6",
            Rank::Seven => "7",
            Rank::Eight => "8",
            Rank::Nine => "9",
            Rank::Ten => "T",
            Rank::Jack => "J",
            Rank::Queen => "Q",
            Rank::King => "K",
            Rank::Ace => "A",
        };
        let suit_str = match self.suit {
            Suit::Hearts => "♥",
            Suit::Diamonds => "♦",
            Suit::Clubs => "♣",
            Suit::Spades => "♠",
        };
        write!(f, "{rank_str}{suit_str}")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Deck {
    pub(crate) cards: Vec<Card>,
}

impl Deck {
    #[must_use]
    pub fn new() -> Self {
        let mut deck = Self {
            cards: Vec::with_capacity(52),
        };
        for suit in [Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades] {
            for rank in [
                Rank::Two,
                Rank::Three,
                Rank::Four,
                Rank::Five,
                Rank::Six,
                Rank::Seven,
                Rank::Eight,
                Rank::Nine,
                Rank::Ten,
                Rank::Jack,
                Rank::Queen,
                Rank::King,
                Rank::Ace,
            ] {
                deck.cards.push(Card::new(suit, rank));
            }
        }
        deck.shuffle();
        deck
    }

    pub fn shuffle(&mut self) {
        use rand::seq::SliceRandom;
        use rand::thread_rng;
        self.cards.shuffle(&mut thread_rng());
    }

    pub fn deal(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    #[must_use]
    pub fn remaining(&self) -> usize {
        self.cards.len()
    }
}

impl Default for Deck {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deck_creation() {
        let deck = Deck::new();
        assert_eq!(deck.remaining(), 52);
    }

    #[test]
    fn test_deck_shuffle_and_deal() {
        let mut deck = Deck::new();
        deck.shuffle();

        let first_card = deck.deal();
        assert!(first_card.is_some());
        assert_eq!(deck.remaining(), 51);

        // Deal all cards
        for _ in 0..51 {
            deck.deal();
        }

        assert_eq!(deck.remaining(), 0);
        assert!(deck.deal().is_none());
    }

    #[test]
    fn test_rank_ordering() {
        assert!(Rank::Ace > Rank::King);
        assert!(Rank::King > Rank::Queen);
        assert!(Rank::Two < Rank::Three);
    }

    #[test]
    fn test_deck_all_unique() {
        let deck = Deck::new();
        let mut seen = std::collections::HashSet::new();
        for card in &deck.cards {
            let key = (card.suit, card.rank);
            assert!(!seen.contains(&key), "Duplicate card found: {:?}", card);
            seen.insert(key);
        }
        assert_eq!(seen.len(), 52);
    }

    #[test]
    fn test_card_display() {
        let card = Card::new(Suit::Hearts, Rank::Ace);
        assert_eq!(format!("{}", card), "A♥");
    }
}
