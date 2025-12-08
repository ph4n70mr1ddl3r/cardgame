#pragma once
#include "bot_state.hpp"
#include "../common/protocol.hpp"
#include <random>
#include <iostream>

namespace poker {

class Strategy {
public:
    static ActionPayload decide(const BotState& state) {
        if (state.valid_actions.empty()) return {ActionType::FOLD, 0};
        
        static std::random_device rd;
        static std::mt19937 g(rd());
        
        // Pick random valid action.
        std::uniform_int_distribution<> dist(0, state.valid_actions.size() - 1);
        ActionType action = state.valid_actions[dist(g)];
        
        int amount = 0;
        if (action == ActionType::RAISE || action == ActionType::BET) {
             // Protocol expects TOTAL amount for BET/RAISE.
             // We'll just do min raise for random bot.
             amount = state.current_bet + state.min_raise;
        } 
        
        return {action, amount};
    }
};

} // namespace poker
