#include "gtest/gtest.h"
#include "../../src/server/table.hpp"
#include "../../src/server/player.hpp"
#include "../../src/server/session.hpp"

using namespace poker;

class MockSession : public GameSession {
public:
    void send(const std::string& message) override {}
    void close() override {}
};

class TableTest : public ::testing::Test {
protected:
    void SetUp() override {
        table = std::make_unique<Table>();
        p1 = std::make_shared<Player>("p1", std::make_shared<MockSession>());
        p2 = std::make_shared<Player>("p2", std::make_shared<MockSession>());
    }

    std::unique_ptr<Table> table;
    std::shared_ptr<Player> p1;
    std::shared_ptr<Player> p2;
};

TEST_F(TableTest, AddRemovePlayer) {
    EXPECT_TRUE(table->addPlayer(p1));
    EXPECT_EQ(table->playerCount(), 1);
    
    EXPECT_TRUE(table->addPlayer(p2));
    EXPECT_EQ(table->playerCount(), 2);
    
    // Table full (Heads Up = 2 players max)
    auto p3 = std::make_shared<Player>("p3", std::make_shared<MockSession>());
    EXPECT_FALSE(table->addPlayer(p3));
    
    table->removePlayer("p1");
    EXPECT_EQ(table->playerCount(), 1);
    EXPECT_EQ(table->getPlayer("p1"), nullptr);
    EXPECT_NE(table->getPlayer("p2"), nullptr);
}

TEST_F(TableTest, PotCalculation) {
    table->addPlayer(p1);
    table->addPlayer(p2);
    
    p1->current_bet = 10;
    p2->current_bet = 20;
    table->pot = 5;
    
    table->collectBetsToPot();
    
    EXPECT_EQ(table->pot, 35); // 5 + 10 + 20
    EXPECT_EQ(p1->current_bet, 0);
    EXPECT_EQ(p2->current_bet, 0);
}

TEST_F(TableTest, DeckHandling) {
    table->resetDeck();
    // Deck should be full after reset
    // We can't check deck size directly as it's not exposed, 
    // but we can check if dealing cards works.
    
    table->addPlayer(p1);
    table->addPlayer(p2);
    
    table->dealHoleCards();
    EXPECT_EQ(p1->hole_cards.size(), 2);
    EXPECT_EQ(p2->hole_cards.size(), 2);
    
    table->dealFlop();
    EXPECT_EQ(table->board.size(), 3);
    
    table->dealTurn();
    EXPECT_EQ(table->board.size(), 4);
    
    table->dealRiver();
    EXPECT_EQ(table->board.size(), 5);
}

TEST_F(TableTest, TopUp) {
    table->addPlayer(p1);
    p1->stack = 0;
    
    // Assuming 5BB is the threshold as per tasks.md (T033/US2)
    // But table.cpp logic might implement "canTopUp"
    
    // Let's check table state logic.
    // If state is waiting for players or ready to start, top up might be allowed.
    table->state = GameState::WAITING_FOR_PLAYERS;
    
    EXPECT_TRUE(table->canTopUp(p1->id));
    table->topUpPlayer(p1->id);
    EXPECT_EQ(p1->stack, 100);
}
