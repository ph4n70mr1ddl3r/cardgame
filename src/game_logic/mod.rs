//! Game logic module for poker rules and mechanics.
//!
//! This module implements the core poker game logic including:
//! - Hand evaluation and ranking
//! - Card dealing and shuffling
//! - Betting rules and validation
//! - Game state transitions

pub mod betting;
pub mod dealer;
pub mod hand_evaluator;

pub use betting::BettingRules;
pub use dealer::Dealer;
pub use hand_evaluator::{evaluate_hand, EvaluatedHand, HandRank};
