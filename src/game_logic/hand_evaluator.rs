use crate::error::{PokerError, Result};
use crate::models::card::{Card, Rank};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum HandRank {
    HighCard = 1,
    Pair = 2,
    TwoPair = 3,
    ThreeOfAKind = 4,
    Straight = 5,
    Flush = 6,
    FullHouse = 7,
    FourOfAKind = 8,
    StraightFlush = 9,
    RoyalFlush = 10,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvaluatedHand {
    pub hand_rank: HandRank,
    pub rank_values: Vec<u8>, // For tie-breaking
    pub description: String,
}

impl PartialOrd for EvaluatedHand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for EvaluatedHand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.hand_rank.cmp(&other.hand_rank) {
            std::cmp::Ordering::Equal => self.rank_values.cmp(&other.rank_values),
            ord => ord,
        }
    }
}

/// Evaluates a 7-card poker hand (2 hole cards + 5 community cards) and returns the best 5-card hand.
///
/// This function examines all 21 possible 5-card combinations and returns the strongest one according
/// to poker hand rankings. It handles all 10 standard poker hands with proper tie-breaking.
///
/// # Arguments
///
/// * `cards` - A vector of exactly 7 cards (2 hole + 5 community)
///
/// # Returns
///
/// * `Result<EvaluatedHand>` - The best possible 5-card hand with ranking information
///
/// # Errors
///
/// Returns an error if the input does not contain exactly 7 cards
pub fn evaluate_hand(mut cards: Vec<Card>) -> Result<EvaluatedHand> {
    if cards.len() != 7 {
        return Err(PokerError::Game(format!(
            "Must have exactly 7 cards (2 hole + 5 community), got {}",
            cards.len()
        )));
    }

    cards.sort_by(|a, b| b.rank.cmp(&a.rank));

    let mut best_hand = EvaluatedHand {
        hand_rank: HandRank::HighCard,
        rank_values: vec![],
        description: String::new(),
    };
    for indices in combinations_indices(7, 5) {
        let combo: Vec<Card> = indices.iter().map(|&i| cards[i]).collect();
        let eval = evaluate_five_cards(&combo);
        if eval > best_hand {
            best_hand = eval;
        }
    }

    Ok(best_hand)
}

fn evaluate_five_cards(cards: &[Card]) -> EvaluatedHand {
    assert_eq!(
        cards.len(),
        5,
        "evaluate_five_cards: Expected exactly 5 cards, got {}",
        cards.len()
    );

    let is_flush = cards.iter().all(|c| c.suit == cards[0].suit);
    let is_straight = check_straight(cards);

    let mut rank_counts: HashMap<Rank, usize> = HashMap::new();
    for card in cards {
        *rank_counts.entry(card.rank).or_insert(0) += 1;
    }

    let mut counts: Vec<(usize, Rank)> = rank_counts
        .iter()
        .map(|(rank, count)| (*count, *rank))
        .collect();
    counts.sort_by(|a, b| match b.0.cmp(&a.0) {
        std::cmp::Ordering::Equal => b.1.cmp(&a.1),
        ord => ord,
    });

    // Royal Flush: A-K-Q-J-10 all same suit
    if is_straight && is_flush && cards[0].rank == Rank::Ace && cards[4].rank == Rank::Ten {
        return EvaluatedHand {
            hand_rank: HandRank::RoyalFlush,
            rank_values: vec![Rank::Ace as u8],
            description: "Royal Flush".to_string(),
        };
    }

    // Straight Flush
    if is_straight && is_flush {
        let high_card = cards[0].rank as u8;
        return EvaluatedHand {
            hand_rank: HandRank::StraightFlush,
            rank_values: vec![high_card],
            description: format!("{}-high Straight Flush", rank_name(cards[0].rank)),
        };
    }

    // Four of a Kind
    if counts[0].0 == 4 {
        return EvaluatedHand {
            hand_rank: HandRank::FourOfAKind,
            rank_values: vec![counts[0].1 as u8, counts[1].1 as u8],
            description: format!("Four {}s", rank_name(counts[0].1)),
        };
    }

    // Full House
    if counts[0].0 == 3 && counts[1].0 == 2 {
        return EvaluatedHand {
            hand_rank: HandRank::FullHouse,
            rank_values: vec![counts[0].1 as u8, counts[1].1 as u8],
            description: format!(
                "{}s full of {}s",
                rank_name(counts[0].1),
                rank_name(counts[1].1)
            ),
        };
    }

    // Flush
    if is_flush {
        let rank_values: Vec<u8> = cards.iter().map(|c| c.rank as u8).collect();
        return EvaluatedHand {
            hand_rank: HandRank::Flush,
            rank_values,
            description: format!("{}-high Flush", rank_name(cards[0].rank)),
        };
    }

    // Straight
    if is_straight {
        let high_card = cards[0].rank as u8;
        return EvaluatedHand {
            hand_rank: HandRank::Straight,
            rank_values: vec![high_card],
            description: format!("{}-high Straight", rank_name(cards[0].rank)),
        };
    }

    // Three of a Kind
    if counts[0].0 == 3 {
        return EvaluatedHand {
            hand_rank: HandRank::ThreeOfAKind,
            rank_values: vec![counts[0].1 as u8, counts[1].1 as u8, counts[2].1 as u8],
            description: format!("Three {}s", rank_name(counts[0].1)),
        };
    }

    // Two Pair
    if counts[0].0 == 2 && counts[1].0 == 2 {
        return EvaluatedHand {
            hand_rank: HandRank::TwoPair,
            rank_values: vec![counts[0].1 as u8, counts[1].1 as u8, counts[2].1 as u8],
            description: format!(
                "Two Pair, {}s and {}s",
                rank_name(counts[0].1),
                rank_name(counts[1].1)
            ),
        };
    }

    // One Pair
    if counts[0].0 == 2 {
        return EvaluatedHand {
            hand_rank: HandRank::Pair,
            rank_values: vec![
                counts[0].1 as u8,
                counts[1].1 as u8,
                counts[2].1 as u8,
                counts[3].1 as u8,
            ],
            description: format!("Pair of {}s", rank_name(counts[0].1)),
        };
    }

    // High Card
    let rank_values: Vec<u8> = cards.iter().map(|c| c.rank as u8).collect();
    EvaluatedHand {
        hand_rank: HandRank::HighCard,
        rank_values,
        description: format!("{}-high", rank_name(cards[0].rank)),
    }
}

/// Checks if a set of cards forms a straight.
///
/// Handles both regular straights and the special "wheel" straight (A-2-3-4-5).
///
/// # Arguments
///
/// * `cards` - A slice of 5 cards sorted in descending order
///
/// # Returns
///
/// * `bool` - True if the cards form a straight, false otherwise
fn check_straight(cards: &[Card]) -> bool {
    let values: Vec<u8> = cards.iter().map(|c| c.rank as u8).collect();

    // Check regular straight
    if values.windows(2).all(|w| w[0] == w[1] + 1) {
        return true;
    }

    // Check A-2-3-4-5 (wheel)
    if values == vec![14, 5, 4, 3, 2] {
        return true;
    }

    false
}

/// Generates all possible combinations of k indices from n items.
///
/// # Arguments
///
/// * `n` - Total number of items
/// * `k` - Size of each combination
///
/// # Returns
///
/// * `Vec<Vec<usize>>` - All combinations of indices
fn combinations_indices(n: usize, k: usize) -> Vec<Vec<usize>> {
    if k > n {
        return vec![];
    }

    let mut result = Vec::new();
    let mut combo = Vec::new();
    combine_indices_helper(n, k, 0, &mut combo, &mut result);
    result
}

/// Recursive helper function for generating combinations.
///
/// # Arguments
///
/// * `n` - Total number of items
/// * `k` - Desired combination size
/// * `start` - Starting index for current recursion level
/// * `combo` - Current combination being built
/// * `result` - Accumulator for all complete combinations
fn combine_indices_helper(
    n: usize,
    k: usize,
    start: usize,
    combo: &mut Vec<usize>,
    result: &mut Vec<Vec<usize>>,
) {
    if combo.len() == k {
        result.push(combo.clone());
        return;
    }

    for i in start..n {
        combo.push(i);
        combine_indices_helper(n, k, i + 1, combo, result);
        combo.pop();
    }
}

/// Returns the human-readable name of a card rank.
///
/// # Arguments
///
/// * `rank` - The rank to get the name for
///
/// # Returns
///
/// * `&'static str` - The rank name as a string slice
fn rank_name(rank: Rank) -> &'static str {
    match rank {
        Rank::Two => "Two",
        Rank::Three => "Three",
        Rank::Four => "Four",
        Rank::Five => "Five",
        Rank::Six => "Six",
        Rank::Seven => "Seven",
        Rank::Eight => "Eight",
        Rank::Nine => "Nine",
        Rank::Ten => "Ten",
        Rank::Jack => "Jack",
        Rank::Queen => "Queen",
        Rank::King => "King",
        Rank::Ace => "Ace",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::card::Suit;

    fn make_card(rank: Rank, suit: Suit) -> Card {
        Card::new(suit, rank)
    }

    #[test]
    fn test_royal_flush() {
        let cards = vec![
            make_card(Rank::Ace, Suit::Hearts),
            make_card(Rank::King, Suit::Hearts),
            make_card(Rank::Queen, Suit::Hearts),
            make_card(Rank::Jack, Suit::Hearts),
            make_card(Rank::Ten, Suit::Hearts),
            make_card(Rank::Two, Suit::Clubs),
            make_card(Rank::Three, Suit::Clubs),
        ];

        let eval = evaluate_hand(cards).unwrap();
        assert_eq!(eval.hand_rank, HandRank::RoyalFlush);
    }

    #[test]
    fn test_straight_flush() {
        let cards = vec![
            make_card(Rank::Nine, Suit::Diamonds),
            make_card(Rank::Eight, Suit::Diamonds),
            make_card(Rank::Seven, Suit::Diamonds),
            make_card(Rank::Six, Suit::Diamonds),
            make_card(Rank::Five, Suit::Diamonds),
            make_card(Rank::Two, Suit::Clubs),
            make_card(Rank::Three, Suit::Clubs),
        ];

        let eval = evaluate_hand(cards).unwrap();
        assert_eq!(eval.hand_rank, HandRank::StraightFlush);
    }

    #[test]
    fn test_four_of_a_kind() {
        let cards = vec![
            make_card(Rank::King, Suit::Hearts),
            make_card(Rank::King, Suit::Diamonds),
            make_card(Rank::King, Suit::Clubs),
            make_card(Rank::King, Suit::Spades),
            make_card(Rank::Ace, Suit::Hearts),
            make_card(Rank::Two, Suit::Clubs),
            make_card(Rank::Three, Suit::Clubs),
        ];

        let eval = evaluate_hand(cards).unwrap();
        assert_eq!(eval.hand_rank, HandRank::FourOfAKind);
    }

    #[test]
    fn test_full_house() {
        let cards = vec![
            make_card(Rank::Ten, Suit::Hearts),
            make_card(Rank::Ten, Suit::Diamonds),
            make_card(Rank::Ten, Suit::Clubs),
            make_card(Rank::Seven, Suit::Spades),
            make_card(Rank::Seven, Suit::Hearts),
            make_card(Rank::Two, Suit::Clubs),
            make_card(Rank::Three, Suit::Clubs),
        ];

        let eval = evaluate_hand(cards).unwrap();
        assert_eq!(eval.hand_rank, HandRank::FullHouse);
    }

    #[test]
    fn test_flush() {
        let cards = vec![
            make_card(Rank::Ace, Suit::Spades),
            make_card(Rank::Jack, Suit::Spades),
            make_card(Rank::Nine, Suit::Spades),
            make_card(Rank::Six, Suit::Spades),
            make_card(Rank::Two, Suit::Spades),
            make_card(Rank::King, Suit::Hearts),
            make_card(Rank::Queen, Suit::Hearts),
        ];

        let eval = evaluate_hand(cards).unwrap();
        assert_eq!(eval.hand_rank, HandRank::Flush);
    }

    #[test]
    fn test_straight() {
        let cards = vec![
            make_card(Rank::Ten, Suit::Hearts),
            make_card(Rank::Nine, Suit::Diamonds),
            make_card(Rank::Eight, Suit::Clubs),
            make_card(Rank::Seven, Suit::Spades),
            make_card(Rank::Six, Suit::Hearts),
            make_card(Rank::Two, Suit::Clubs),
            make_card(Rank::Ace, Suit::Diamonds),
        ];

        let eval = evaluate_hand(cards).unwrap();
        assert_eq!(eval.hand_rank, HandRank::Straight);
    }

    #[test]
    fn test_wheel_straight() {
        // A-2-3-4-5 (wheel/bicycle)
        let cards = vec![
            make_card(Rank::Ace, Suit::Hearts),
            make_card(Rank::Five, Suit::Diamonds),
            make_card(Rank::Four, Suit::Clubs),
            make_card(Rank::Three, Suit::Spades),
            make_card(Rank::Two, Suit::Hearts),
            make_card(Rank::King, Suit::Clubs),
            make_card(Rank::Queen, Suit::Diamonds),
        ];

        let eval = evaluate_hand(cards).unwrap();
        assert_eq!(eval.hand_rank, HandRank::Straight);
    }

    #[test]
    fn test_three_of_a_kind() {
        let cards = vec![
            make_card(Rank::Jack, Suit::Hearts),
            make_card(Rank::Jack, Suit::Diamonds),
            make_card(Rank::Jack, Suit::Clubs),
            make_card(Rank::King, Suit::Spades),
            make_card(Rank::Nine, Suit::Hearts),
            make_card(Rank::Two, Suit::Clubs),
            make_card(Rank::Three, Suit::Diamonds),
        ];

        let eval = evaluate_hand(cards).unwrap();
        assert_eq!(eval.hand_rank, HandRank::ThreeOfAKind);
    }

    #[test]
    fn test_two_pair() {
        let cards = vec![
            make_card(Rank::Queen, Suit::Hearts),
            make_card(Rank::Queen, Suit::Diamonds),
            make_card(Rank::Seven, Suit::Clubs),
            make_card(Rank::Seven, Suit::Spades),
            make_card(Rank::Ace, Suit::Hearts),
            make_card(Rank::Two, Suit::Clubs),
            make_card(Rank::Three, Suit::Diamonds),
        ];

        let eval = evaluate_hand(cards).unwrap();
        assert_eq!(eval.hand_rank, HandRank::TwoPair);
    }

    #[test]
    fn test_pair() {
        let cards = vec![
            make_card(Rank::Eight, Suit::Hearts),
            make_card(Rank::Eight, Suit::Diamonds),
            make_card(Rank::Ace, Suit::Clubs),
            make_card(Rank::King, Suit::Spades),
            make_card(Rank::Queen, Suit::Hearts),
            make_card(Rank::Two, Suit::Clubs),
            make_card(Rank::Three, Suit::Diamonds),
        ];

        let eval = evaluate_hand(cards).unwrap();
        assert_eq!(eval.hand_rank, HandRank::Pair);
    }

    #[test]
    fn test_high_card() {
        let cards = vec![
            make_card(Rank::Ace, Suit::Hearts),
            make_card(Rank::King, Suit::Diamonds),
            make_card(Rank::Jack, Suit::Clubs),
            make_card(Rank::Nine, Suit::Spades),
            make_card(Rank::Seven, Suit::Hearts),
            make_card(Rank::Five, Suit::Clubs),
            make_card(Rank::Three, Suit::Diamonds),
        ];

        let eval = evaluate_hand(cards).unwrap();
        assert_eq!(eval.hand_rank, HandRank::HighCard);
    }

    #[test]
    fn test_hand_comparison() {
        let flush_cards = vec![
            make_card(Rank::Ace, Suit::Spades),
            make_card(Rank::Jack, Suit::Spades),
            make_card(Rank::Nine, Suit::Spades),
            make_card(Rank::Six, Suit::Spades),
            make_card(Rank::Two, Suit::Spades),
            make_card(Rank::King, Suit::Hearts),
            make_card(Rank::Queen, Suit::Hearts),
        ];

        let straight_cards = vec![
            make_card(Rank::Ten, Suit::Hearts),
            make_card(Rank::Nine, Suit::Diamonds),
            make_card(Rank::Eight, Suit::Clubs),
            make_card(Rank::Seven, Suit::Spades),
            make_card(Rank::Six, Suit::Hearts),
            make_card(Rank::Two, Suit::Clubs),
            make_card(Rank::Ace, Suit::Diamonds),
        ];

        let flush_hand = evaluate_hand(flush_cards).unwrap();
        let straight_hand = evaluate_hand(straight_cards).unwrap();

        assert!(flush_hand > straight_hand);
    }
}
