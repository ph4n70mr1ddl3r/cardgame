#include "table.hpp"

namespace poker {

Table::Table() 
    : state(GameState::WAITING_FOR_PLAYERS), 
      pot(0), dealer_pos(0), current_turn(0), min_raise(2), current_bet(0) 
{
    seats.resize(2, nullptr);
}

bool Table::addPlayer(std::shared_ptr<Player> player) {
    for (int i = 0; i < 2; ++i) {
        if (!seats[i]) {
            seats[i] = player;
            player->position = i;
            return true;
        }
    }
    return false;
}

void Table::removePlayer(const std::string& playerId) {
    for (int i = 0; i < 2; ++i) {
        if (seats[i] && seats[i]->id == playerId) {
            seats[i] = nullptr;
            return;
        }
    }
}

std::shared_ptr<Player> Table::getPlayer(const std::string& playerId) {
    for (auto& p : seats) {
        if (p && p->id == playerId) return p;
    }
    return nullptr;
}

std::shared_ptr<Player> Table::getPlayerAt(int pos) {
    if (pos >= 0 && pos < 2) return seats[pos];
    return nullptr;
}

int Table::playerCount() const {
    int count = 0;
    for (auto& p : seats) if (p) count++;
    return count;
}

int Table::activePlayerCount() const {
    int count = 0;
    for (auto& p : seats) if (p && p->status == PlayerStatus::ACTIVE) count++;
    return count;
}

void Table::markDisconnected(const std::string& playerId) {
    auto p = getPlayer(playerId);
    if (p) {
        p->status = PlayerStatus::DISCONNECTED;
    }
}

void Table::markSittingOut(const std::string& playerId) {
    auto p = getPlayer(playerId);
    if (p) {
        p->status = PlayerStatus::SITTING_OUT;
        p->is_folded = true; // Auto-fold if sitting out
    }
}

void Table::markActive(const std::string& playerId) {
    auto p = getPlayer(playerId);
    if (p) {
        p->status = PlayerStatus::ACTIVE;
    }
}

bool Table::canTopUp(const std::string& playerId) {
    auto p = getPlayer(playerId);
    if (!p) return false;
    
    // Allow top up if game is not in active play state regarding this player
    // Safe states: Waiting, Hand End
    if (state == GameState::WAITING_FOR_PLAYERS || state == GameState::HAND_END) return true;
    
    // Also allow if player is not in the current hand (Sitting Out)
    if (p->status == PlayerStatus::SITTING_OUT) return true;
    
    // If folded, technically safe to top up for NEXT hand
    if (p->is_folded) return true;
    
    return false;
}

void Table::topUpPlayer(const std::string& playerId) {
    auto p = getPlayer(playerId);
    if (p) p->stack = 100;
}

void Table::resetDeck() {
    deck.reset();
    deck.shuffle();
}

void Table::dealHoleCards() {
    for (auto& p : seats) {
        if (p && p->status == PlayerStatus::ACTIVE) {
            p->hole_cards.clear();
            p->hole_cards.push_back(deck.draw());
            p->hole_cards.push_back(deck.draw());
            p->is_folded = false;
        }
    }
}

void Table::dealFlop() {
    deck.draw(); // Burn
    board.push_back(deck.draw());
    board.push_back(deck.draw());
    board.push_back(deck.draw());
}

void Table::dealTurn() {
    deck.draw(); // Burn
    board.push_back(deck.draw());
}

void Table::dealRiver() {
    deck.draw(); // Burn
    board.push_back(deck.draw());
}

void Table::collectBetsToPot() {
    for (auto& p : seats) {
        if (p) {
            pot += p->current_bet;
            p->current_bet = 0;
        }
    }
    current_bet = 0;
    min_raise = 2; 
}

void Table::resetHand() {
    board.clear();
    pot = 0;
    current_bet = 0;
    min_raise = 2; 
    for (auto& p : seats) {
        if (p) {
            p->current_bet = 0;
            p->hole_cards.clear();
            p->is_folded = false;
        }
    }
}

} // namespace poker
