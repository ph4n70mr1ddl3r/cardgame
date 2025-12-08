#pragma once
#include "types.hpp"
#include "card.hpp"
#include <nlohmann/json.hpp>
#include <vector>
#include <string>

using json = nlohmann::json;

namespace poker {

// Message Types
inline const std::string MSG_LOGIN = "LOGIN";
inline const std::string MSG_ACTION = "ACTION";
inline const std::string MSG_TOP_UP = "TOP_UP";
inline const std::string MSG_GAME_STATE = "GAME_STATE";
inline const std::string MSG_REQUEST_ACTION = "REQUEST_ACTION";
inline const std::string MSG_HOLE_CARDS = "HOLE_CARDS";
inline const std::string MSG_ERROR = "ERROR";

// Serialization for Enums
NLOHMANN_JSON_SERIALIZE_ENUM(Suit, {
    {Suit::HEARTS, "HEARTS"},
    {Suit::DIAMONDS, "DIAMONDS"},
    {Suit::CLUBS, "CLUBS"},
    {Suit::SPADES, "SPADES"}
})

NLOHMANN_JSON_SERIALIZE_ENUM(Rank, {
    {Rank::TWO, "TWO"}, {Rank::THREE, "THREE"}, {Rank::FOUR, "FOUR"},
    {Rank::FIVE, "FIVE"}, {Rank::SIX, "SIX"}, {Rank::SEVEN, "SEVEN"},
    {Rank::EIGHT, "EIGHT"}, {Rank::NINE, "NINE"}, {Rank::TEN, "TEN"},
    {Rank::JACK, "JACK"}, {Rank::QUEEN, "QUEEN"}, {Rank::KING, "KING"},
    {Rank::ACE, "ACE"}
})

NLOHMANN_JSON_SERIALIZE_ENUM(ActionType, {
    {ActionType::FOLD, "FOLD"},
    {ActionType::CHECK, "CHECK"},
    {ActionType::CALL, "CALL"},
    {ActionType::BET, "BET"},
    {ActionType::RAISE, "RAISE"},
    {ActionType::ALL_IN, "ALL_IN"}
})

NLOHMANN_JSON_SERIALIZE_ENUM(GameState, {
    {GameState::WAITING_FOR_PLAYERS, "WAITING_FOR_PLAYERS"},
    {GameState::PREFLOP, "PREFLOP"},
    {GameState::FLOP, "FLOP"},
    {GameState::TURN, "TURN"},
    {GameState::RIVER, "RIVER"},
    {GameState::SHOWDOWN, "SHOWDOWN"},
    {GameState::HAND_END, "HAND_END"}
})

// Card Serialization
inline void to_json(json& j, const Card& c) {
    j = json{{"rank", c.rank}, {"suit", c.suit}};
}

inline void from_json(const json& j, Card& c) {
    j.at("rank").get_to(c.rank);
    j.at("suit").get_to(c.suit);
}

// Basic Message
struct Message {
    std::string type;
    json payload;
};

inline void to_json(json& j, const Message& m) {
    j = json{{"type", m.type}, {"payload", m.payload}};
}

inline void from_json(const json& j, Message& m) {
    j.at("type").get_to(m.type);
    j.at("payload").get_to(m.payload);
}

// Payloads
struct LoginPayload {
    std::string player_id;
};
NLOHMANN_DEFINE_TYPE_NON_INTRUSIVE(LoginPayload, player_id)

struct ActionPayload {
    ActionType action;
    int amount = 0;
};
NLOHMANN_DEFINE_TYPE_NON_INTRUSIVE(ActionPayload, action, amount)

struct PlayerPublicState {
    std::string id;
    int stack;
    int bet;
    bool is_active;
    bool has_folded;
};
NLOHMANN_DEFINE_TYPE_NON_INTRUSIVE(PlayerPublicState, id, stack, bet, is_active, has_folded)

struct GameStatePayload {
    GameState state;
    int pot;
    std::vector<Card> board;
    std::vector<PlayerPublicState> players;
    int dealer_pos;
    int current_turn;
};
NLOHMANN_DEFINE_TYPE_NON_INTRUSIVE(GameStatePayload, state, pot, board, players, dealer_pos, current_turn)

struct RequestActionPayload {
    std::vector<ActionType> valid_actions;
    int min_raise;
    int current_bet;
    int call_amount;
};
NLOHMANN_DEFINE_TYPE_NON_INTRUSIVE(RequestActionPayload, valid_actions, min_raise, current_bet, call_amount)

struct HoleCardsPayload {
    std::vector<Card> cards;
};
NLOHMANN_DEFINE_TYPE_NON_INTRUSIVE(HoleCardsPayload, cards)

struct ErrorPayload {
    std::string code;
    std::string message;
};
NLOHMANN_DEFINE_TYPE_NON_INTRUSIVE(ErrorPayload, code, message)

} // namespace poker
