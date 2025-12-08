#pragma once

#include <cstdint>
#include <string>

namespace poker {

enum class Suit {
    HEARTS,
    DIAMONDS,
    CLUBS,
    SPADES
};

enum class Rank {
    TWO = 2,
    THREE = 3,
    FOUR = 4,
    FIVE = 5,
    SIX = 6,
    SEVEN = 7,
    EIGHT = 8,
    NINE = 9,
    TEN = 10,
    JACK = 11,
    QUEEN = 12,
    KING = 13,
    ACE = 14
};

enum class GameState {
    WAITING_FOR_PLAYERS,
    PREFLOP,
    FLOP,
    TURN,
    RIVER,
    SHOWDOWN,
    HAND_END
};

enum class PlayerStatus {
    ACTIVE,
    DISCONNECTED,
    SITTING_OUT
};

enum class ActionType {
    FOLD,
    CHECK,
    CALL,
    BET,
    RAISE,
    ALL_IN
};

// String conversions for debugging/logging could be added here
inline std::string actionToString(ActionType type) {
    switch (type) {
        case ActionType::FOLD: return "FOLD";
        case ActionType::CHECK: return "CHECK";
        case ActionType::CALL: return "CALL";
        case ActionType::BET: return "BET";
        case ActionType::RAISE: return "RAISE";
        case ActionType::ALL_IN: return "ALL_IN";
        default: return "UNKNOWN";
    }
}

} // namespace poker
