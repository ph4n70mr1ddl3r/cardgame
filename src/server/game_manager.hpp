#pragma once
#include "table.hpp"
#include "../common/protocol.hpp"
#include "../common/hand_evaluator.hpp"
#include <functional>
#include <string>
#include <vector>
#include "timeout_manager.hpp" // Include TimeoutManager

namespace poker {

class GameManager {
public:
    GameManager(boost::asio::io_context& ioc); // Pass ioc to TimeoutManager
    
    // External events
    void onPlayerJoin(std::shared_ptr<Player> player);
    void onPlayerAction(const std::string& playerId, const ActionPayload& action);
    void onPlayerDisconnect(const std::string& playerId);
    void onPlayerTimeout(const std::string& playerId);
    void onPlayerLeave(const std::string& playerId);
    void onPlayerTopUp(const std::string& playerId);
    
    // Callbacks for IO
    std::function<void(const std::string&, const Message&)> sendToPlayer;
    std::function<void(const Message&)> broadcast;

    Table table;
    TimeoutManager timeout_manager; // Member for managing timeouts

private:
    void startHand();
    void nextStreet();
    void showdown();
    void endHand();
    void distributePot(const std::vector<std::string>& winners);
    
    void requestNextAction();
    void handleFold(std::shared_ptr<Player> player);
    void checkRoundComplete();
    
    bool isRoundComplete() const;
    int playersInHand() const;
    
    // Logic helper
    void broadcastGameState();
};

} // namespace poker
