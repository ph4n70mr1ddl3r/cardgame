#include <gtest/gtest.h>
#include "hand_evaluator.hpp"

using namespace poker;

// Helper to create cards easily
Card C(Rank r, Suit s) { return {r, s}; }

TEST(EvaluatorTest, HighCard) {
    std::vector<Card> board = {C(Rank::TWO, Suit::HEARTS), C(Rank::THREE, Suit::DIAMONDS), C(Rank::FIVE, Suit::SPADES), C(Rank::SEVEN, Suit::CLUBS), C(Rank::NINE, Suit::HEARTS)};
    std::vector<Card> hole = {C(Rank::KING, Suit::HEARTS), C(Rank::QUEEN, Suit::DIAMONDS)};
    
    auto res = HandEvaluator::evaluate(hole, board);
    EXPECT_EQ(res.rank, HandRank::HIGH_CARD);
    EXPECT_EQ(res.kickers[0], Rank::KING);
}

TEST(EvaluatorTest, Pair) {
    std::vector<Card> board = {C(Rank::TWO, Suit::HEARTS), C(Rank::TWO, Suit::DIAMONDS), C(Rank::FIVE, Suit::SPADES), C(Rank::SEVEN, Suit::CLUBS), C(Rank::NINE, Suit::HEARTS)};
    std::vector<Card> hole = {C(Rank::KING, Suit::HEARTS), C(Rank::QUEEN, Suit::DIAMONDS)};
    
    auto res = HandEvaluator::evaluate(hole, board);
    EXPECT_EQ(res.rank, HandRank::PAIR);
    EXPECT_EQ(res.kickers[0], Rank::TWO); // Pair of Twos
    EXPECT_EQ(res.kickers[1], Rank::KING); // Kicker King
}

TEST(EvaluatorTest, Flush) {
    std::vector<Card> board = {C(Rank::TWO, Suit::HEARTS), C(Rank::FOUR, Suit::HEARTS), C(Rank::SIX, Suit::HEARTS), C(Rank::EIGHT, Suit::HEARTS), C(Rank::NINE, Suit::DIAMONDS)};
    std::vector<Card> hole = {C(Rank::KING, Suit::HEARTS), C(Rank::QUEEN, Suit::DIAMONDS)};
    
    auto res = HandEvaluator::evaluate(hole, board);
    EXPECT_EQ(res.rank, HandRank::FLUSH);
    EXPECT_EQ(res.kickers[0], Rank::KING);
}

TEST(EvaluatorTest, Straight) {
    std::vector<Card> board = {C(Rank::TWO, Suit::HEARTS), C(Rank::THREE, Suit::DIAMONDS), C(Rank::FOUR, Suit::SPADES), C(Rank::FIVE, Suit::CLUBS), C(Rank::NINE, Suit::HEARTS)};
    std::vector<Card> hole = {C(Rank::SIX, Suit::HEARTS), C(Rank::QUEEN, Suit::DIAMONDS)};
    
    auto res = HandEvaluator::evaluate(hole, board);
    EXPECT_EQ(res.rank, HandRank::STRAIGHT);
    EXPECT_EQ(res.kickers[0], Rank::SIX);
}

TEST(EvaluatorTest, Comparison) {
    // Flush > Straight
    HandResult flush = {HandRank::FLUSH, {Rank::ACE, Rank::KING, Rank::QUEEN, Rank::JACK, Rank::NINE}};
    HandResult straight = {HandRank::STRAIGHT, {Rank::ACE}};
    
    EXPECT_TRUE(flush > straight);
}
