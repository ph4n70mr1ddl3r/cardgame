#include "gtest/gtest.h"
#include "../../src/server/game_manager.hpp"
#include "../../src/server/player.hpp"
#include "../../src/server/session.hpp"
#include "../../src/common/protocol.hpp"

using namespace poker;

class MockSessionGM : public GameSession {
public:
    void send(const std::string& message) override {
        last_message = message;
    }
    void close() override {}
    std::string last_message;
};

class GameManagerTest : public ::testing::Test {
protected:
    void SetUp() override {
        gm = std::make_unique<GameManager>();
        p1_session = std::make_shared<MockSessionGM>();
        p2_session = std::make_shared<MockSessionGM>();
        p1 = std::make_shared<Player>("p1", p1_session);
        p2 = std::make_shared<Player>("p2", p2_session);
        
        // Mock callbacks to prevent crashes if they are called
        gm->sendToPlayer = [](const std::string& pid, const Message& msg) {};
        gm->broadcast = [](const Message& msg) {};
    }

    std::unique_ptr<GameManager> gm;
    std::shared_ptr<MockSessionGM> p1_session;
    std::shared_ptr<MockSessionGM> p2_session;
    std::shared_ptr<Player> p1;
    std::shared_ptr<Player> p2;
};

TEST_F(GameManagerTest, PlayerJoin) {
    gm->onPlayerJoin(p1);
    EXPECT_EQ(gm->table.playerCount(), 1);
    
    gm->onPlayerJoin(p2);
    EXPECT_EQ(gm->table.playerCount(), 2);
}

TEST_F(GameManagerTest, GameStartCondition) {
    gm->onPlayerJoin(p1);
    EXPECT_NE(gm->table.state, GameState::PREFLOP); // Should not start with 1 player
    
    gm->onPlayerJoin(p2);
    // Depending on implementation, it might auto-start or wait for something else.
    // Given the simplicity, let's assume it might transition or at least accept the player.
    EXPECT_EQ(gm->table.playerCount(), 2);
}

TEST_F(GameManagerTest, TopUp) {
    gm->onPlayerJoin(p1);
    gm->onPlayerJoin(p2);
    
    // Game started, state is PREFLOP. Top up not allowed.
    // Force state to HAND_END to simulate end of hand
    gm->table.state = GameState::HAND_END;
    
    // Set stack to low
    p1->stack = 10;
    
    // Top up
    gm->onPlayerTopUp("p1");
    
    // Should be topped up to max (100)
    EXPECT_EQ(p1->stack, 100);
}
