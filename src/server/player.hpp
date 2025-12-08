#pragma once
#include "../common/types.hpp"
#include "../common/card.hpp"
#include "session.hpp"
#include <string>
#include <vector>
#include <memory>

namespace poker {

class Player {
public:
    Player(std::string id, std::shared_ptr<GameSession> session)
        : id(std::move(id)), stack(100), current_bet(0), status(PlayerStatus::ACTIVE), is_folded(false), position(-1), session(session) {}

    std::string id;
    int stack; // In BB
    int current_bet;
    std::vector<Card> hole_cards;
    PlayerStatus status;
    bool is_folded;
    int position; 

    std::weak_ptr<GameSession> session;
};

} // namespace poker
