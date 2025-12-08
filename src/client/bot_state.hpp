#pragma once
#include "../common/types.hpp"
#include "../common/card.hpp"
#include <string>
#include <vector>

namespace poker {

struct BotState {
    std::string my_id;
    int my_stack = 0;
    std::vector<Card> my_hole_cards;
    
    GameState game_state = GameState::WAITING_FOR_PLAYERS;
    int pot = 0;
    std::vector<Card> board;
    int current_bet = 0;
    int min_raise = 0;
    
    bool is_my_turn = false;
    std::vector<ActionType> valid_actions;
    int call_amount = 0;
};

} // namespace poker
