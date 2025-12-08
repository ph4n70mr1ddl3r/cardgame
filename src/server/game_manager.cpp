#include "game_manager.hpp"
#include <iostream>
#include <algorithm>

namespace poker {

GameManager::GameManager() {}

void GameManager::onPlayerJoin(std::shared_ptr<Player> player) {
    if (table.addPlayer(player)) {
        std::cout << "Player joined: " << player->id << " at pos " << player->position << std::endl;
        broadcastGameState();
        
        if (table.activePlayerCount() == 2 && table.state == GameState::WAITING_FOR_PLAYERS) {
            std::cout << "Starting game..." << std::endl;
            startHand();
        }
    }
}

void GameManager::onPlayerDisconnect(const std::string& playerId) {
    auto p = table.getPlayer(playerId);
    if (p) {
        p->status = PlayerStatus::DISCONNECTED;
        broadcastGameState();
        // Timeout logic handled elsewhere (US3)
    }
}

void GameManager::broadcastGameState() {
    if (!broadcast) return;

    GameStatePayload pl;
    pl.state = table.state;
    pl.pot = table.pot;
    pl.board = table.board;
    pl.dealer_pos = table.dealer_pos;
    pl.current_turn = table.current_turn;

    for (const auto& p : table.seats) {
        if (p) {
            pl.players.push_back({
                p->id,
                p->stack,
                p->current_bet,
                p->status == PlayerStatus::ACTIVE,
                p->is_folded
            });
        }
    }
    
    Message msg;
    msg.type = MSG_GAME_STATE;
    msg.payload = pl; // Auto serialization
    broadcast(msg);
}

void GameManager::startHand() {
    table.resetHand();
    table.resetDeck();
    table.state = GameState::PREFLOP;

    // Rotate dealer if not first hand? For now simple logic.
    // In Heads Up: Dealer (SB) = 0, Big Blind = 1.
    // SB posts 1, BB posts 2.
    // Preflop action starts with Dealer (SB).
    // Postflop action starts with BB.

    auto sb = table.seats[table.dealer_pos];
    auto bb = table.seats[1 - table.dealer_pos];

    if (!sb || !bb) return;

    // Post Blinds
    int sb_amount = 1;
    int bb_amount = 2;

    sb->stack -= sb_amount;
    sb->current_bet = sb_amount;
    
    bb->stack -= bb_amount;
    bb->current_bet = bb_amount;
    
    table.current_bet = 2; // BB amount
    table.min_raise = 2;   // 1 BB raise
    
    table.dealHoleCards();
    
    // Send Hole Cards
    for (auto& p : table.seats) {
        if (p && sendToPlayer) {
            HoleCardsPayload hcp;
            hcp.cards = p->hole_cards;
            Message m;
            m.type = MSG_HOLE_CARDS;
            m.payload = hcp;
            sendToPlayer(p->id, m);
        }
    }

    broadcastGameState();

    // Set turn to Dealer (SB) for preflop
    table.current_turn = table.dealer_pos;
    requestNextAction();
}

void GameManager::requestNextAction() {
    auto p = table.getPlayerAt(table.current_turn);
    if (!p || !sendToPlayer) return;

    RequestActionPayload rap;
    rap.current_bet = table.current_bet;
    rap.call_amount = table.current_bet - p->current_bet;
    rap.min_raise = table.min_raise;
    
    // Determine valid actions
    rap.valid_actions.push_back(ActionType::FOLD);
    if (rap.call_amount == 0) rap.valid_actions.push_back(ActionType::CHECK);
    else rap.valid_actions.push_back(ActionType::CALL);
    
    // Can Raise/Bet?
    if (p->stack > rap.call_amount) {
        rap.valid_actions.push_back(ActionType::RAISE); // or BET
        rap.valid_actions.push_back(ActionType::ALL_IN);
    }

    Message m;
    m.type = MSG_REQUEST_ACTION;
    m.payload = rap;
    sendToPlayer(p->id, m);
}

void GameManager::onPlayerAction(const std::string& playerId, const ActionPayload& action) {
    auto p = table.getPlayer(playerId);
    if (!p || table.seats[table.current_turn] != p) {
        // Not your turn or invalid player
        return; 
    }

    std::cout << "Action from " << playerId << ": " << actionToString(action.action) << " " << action.amount << std::endl;

    // Validate and Apply Action (simplified validation)
    bool action_ok = true;
    
    switch (action.action) {
        case ActionType::FOLD:
            handleFold(p);
            return; // Hand ends immediately in Heads Up if fold
        case ActionType::CHECK:
            if (table.current_bet > p->current_bet) action_ok = false;
            break;
        case ActionType::CALL:
        {
            int to_call = table.current_bet - p->current_bet;
            if (p->stack < to_call) {
                // All in via call
                p->current_bet += p->stack;
                p->stack = 0;
            } else {
                p->current_bet += to_call;
                p->stack -= to_call;
            }
            break;
        }
        case ActionType::BET:
        case ActionType::RAISE:
        {
            int total_bet = action.amount;
            if (total_bet < table.current_bet + table.min_raise && total_bet < p->stack + p->current_bet) {
                 // Invalid raise amount, unless all in. Simplified: accept if >= min raise
                 // Assuming action.amount is TOTAL bet (current_bet + added)
            }
            int added = total_bet - p->current_bet;
            if (p->stack < added) {
                // Not enough chips
                action_ok = false;
            } else {
                p->stack -= added;
                p->current_bet = total_bet;
                if (total_bet > table.current_bet) {
                    // Update min raise (simplified)
                    table.min_raise = total_bet - table.current_bet;
                    table.current_bet = total_bet;
                }
            }
            break;
        }
        case ActionType::ALL_IN:
            p->current_bet += p->stack;
            p->stack = 0;
            if (p->current_bet > table.current_bet) {
                table.min_raise = p->current_bet - table.current_bet;
                table.current_bet = p->current_bet;
            }
            break;
    }

    if (!action_ok) {
        // Send error
        if (sendToPlayer) {
            Message m; m.type = MSG_ERROR; m.payload = ErrorPayload{"INVALID_MOVE", "Invalid move"};
            sendToPlayer(p->id, m);
        }
        // Don't advance turn
        return;
    }

    broadcastGameState();
    checkRoundComplete();
}

void GameManager::handleFold(std::shared_ptr<Player> p) {
    p->is_folded = true;
    broadcastGameState();
    
    // In Heads Up, other player wins immediately
    auto winner = table.seats[1 - p->position];
    if (winner) {
        distributePot({winner->id});
    }
}

void GameManager::checkRoundComplete() {
    // Round is complete if:
    // 1. All active players have acted (we need to track who acted this street, simplified here)
    // 2. Bets are equal (ignoring all-ins)
    
    auto p1 = table.seats[0];
    auto p2 = table.seats[1];
    
    if (!p1 || !p2) return;
    
    // In strict sense, we need to know if the current player raised or just called.
    // Simple heuristic: if bets equal and both acted at least once? 
    // Or simpler: We switch turn. If we come back to a player and bets are equal, round done?
    // Let's iterate. 
    // Next turn logic:
    
    bool bets_equal = (p1->current_bet == p2->current_bet);
    // Special case: All in.
    
    // If bets are equal, AND it wasn't just the Big Blind option preflop.
    // Preflop: SB(Dealer) acts. If calls (matches BB), BB has option.
    // If BB checks, round over. If BB raises, continues.
    
    // Simplified State Machine:
    // We pass turn to next player.
    // If next player is all-in or folded?
    
    int next_pos = 1 - table.current_turn;
    
    // Very simplified round end check:
    // If current action was CALL/CHECK and bets equal: Round End.
    // If current action was BET/RAISE: Not End.
    // Warning: This is tricky.
    
    // Let's assume onPlayerAction handles the logic? 
    // If the action made bets equal (CALL or CHECK), and it wasn't the "opening" of the round where checking is allowed but doesn't end (BB preflop option).
    
    // For MVP:
    // If bets equal:
    //   If Preflop and p->current_bet == 2 (BB) and actor was SB: BB needs to act (Option).
    //   Else: End Street.
    
    if (bets_equal) {
        if (table.state == GameState::PREFLOP && table.current_bet == 2 && table.current_turn == table.dealer_pos) {
             // SB just called. Pass to BB.
             table.current_turn = next_pos;
             requestNextAction();
        } else {
             nextStreet();
        }
    } else {
        // Bets not equal, pass turn
        table.current_turn = next_pos;
        requestNextAction();
    }
}

void GameManager::nextStreet() {
    table.collectBetsToPot();
    
    switch (table.state) {
        case GameState::PREFLOP:
            table.state = GameState::FLOP;
            table.dealFlop();
            break;
        case GameState::FLOP:
            table.state = GameState::TURN;
            table.dealTurn();
            break;
        case GameState::TURN:
            table.state = GameState::RIVER;
            table.dealRiver();
            break;
        case GameState::RIVER:
            showdown();
            return;
        default:
            return;
    }
    
    broadcastGameState();
    
    // Post-flop action starts with non-dealer (BB) -> position 1-dealer_pos
    table.current_turn = 1 - table.dealer_pos;
    requestNextAction();
}

void GameManager::showdown() {
    table.state = GameState::SHOWDOWN;
    broadcastGameState();
    
    auto p1 = table.seats[0];
    auto p2 = table.seats[1];
    
    if (!p1 || !p2) { endHand(); return; } // Should not happen
    
    auto h1 = HandEvaluator::evaluate(p1->hole_cards, table.board);
    auto h2 = HandEvaluator::evaluate(p2->hole_cards, table.board);
    
    std::cout << "Showdown! P1 Rank: " << (int)h1.rank << ", P2 Rank: " << (int)h2.rank << std::endl;
    
    if (h1 > h2) distributePot({p1->id});
    else if (h2 > h1) distributePot({p2->id});
    else distributePot({p1->id, p2->id}); // Split
}

void GameManager::distributePot(const std::vector<std::string>& winners) {
    // Add current bets to pot before distribution (if folded or showdown)
    table.collectBetsToPot();

    int share = table.pot / winners.size();
    for (const auto& w_id : winners) {
        auto p = table.getPlayer(w_id);
        if (p) p->stack += share;
    }
    // Odd chip goes to... dealer? First winner? Ignore for MVP.
    
    endHand();
}

void GameManager::endHand() {
    table.state = GameState::HAND_END;
    broadcastGameState();
    
    // Rotate Dealer
    table.dealer_pos = 1 - table.dealer_pos;
    
    // Wait a bit? Or start immediately.
    // For now immediately start next hand if players valid.
    
    if (table.activePlayerCount() == 2) {
        startHand();
    }
}

} // namespace poker
