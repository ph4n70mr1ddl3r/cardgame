#pragma once
#include <string>

namespace poker {

class GameSession {
public:
    virtual ~GameSession() = default;
    virtual void send(const std::string& message) = 0;
    virtual void close() = 0;
};

} // namespace poker
