#include <gtest/gtest.h>
#include "card.hpp"
#include <set>

using namespace poker;

TEST(DeckTest, InitialSize) {
    Deck deck;
    int count = 0;
    while (!deck.isEmpty()) {
        deck.draw();
        count++;
    }
    EXPECT_EQ(count, 52);
}

TEST(DeckTest, UniqueCards) {
    Deck deck;
    std::set<std::pair<int, int>> drawn_cards;
    
    for (int i = 0; i < 52; ++i) {
        Card c = deck.draw();
        drawn_cards.insert({static_cast<int>(c.rank), static_cast<int>(c.suit)});
    }
    
    EXPECT_EQ(drawn_cards.size(), 52);
}

TEST(DeckTest, ShuffleChangesOrder) {
    Deck deck1;
    deck1.shuffle();
    
    Deck deck2;
    deck2.shuffle();
    
    // It's technically possible but highly unlikely that two shuffled decks are identical.
    // We'll just check the first few cards.
    bool all_same = true;
    for (int i = 0; i < 10; ++i) {
        if (!(deck1.draw() == deck2.draw())) {
            all_same = false;
            break;
        }
    }
    EXPECT_FALSE(all_same);
}

TEST(DeckTest, ResetRefillsDeck) {
    Deck deck;
    deck.draw();
    deck.reset();
    
    int count = 0;
    while (!deck.isEmpty()) {
        deck.draw();
        count++;
    }
    EXPECT_EQ(count, 52);
}
