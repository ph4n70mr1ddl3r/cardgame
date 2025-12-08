#pragma once
#include "player.hpp"
#include "../common/types.hpp"
#include "../common/card.hpp"
#include <vector>
#include <memory>
#include <algorithm>

namespace poker {

class Table {
public:
    Table();
    
    // Player management
    bool addPlayer(std::shared_ptr<Player> player);
    void removePlayer(const std::string& playerId);
    std::shared_ptr<Player> getPlayer(const std::string& playerId);
    std::shared_ptr<Player> getPlayerAt(int pos);
    int playerCount() const;
    int activePlayerCount() const;

    // Game Actions
    void resetDeck();
    void dealHoleCards();
    void dealFlop();
    void dealTurn();
    void dealRiver();
    
    // State
    GameState state;
    Deck deck;
    std::vector<Card> board;
    int pot;
    int dealer_pos;
    int current_turn; // Position of player acting
    int min_raise;
    int current_bet; // Highest bet in current street

    // Players by position (0, 1) - fixed size 2 for HU
    std::vector<std::shared_ptr<Player>> seats;

    // Helpers
    void collectBetsToPot();
    void resetHand(); 
};

} // namespace poker
