# Research: Heads Up NLHE Server and Bot

## Technical Decisions

### 1. Networking & Concurrency
*   **Decision**: Single-threaded Asynchronous I/O using `Boost.Asio` and `Boost.Beast`.
*   **Rationale**:
    *   **Simplicity**: Avoids complex mutex locking for game state since all logic runs on a single `io_context` strand.
    *   **Performance**: More than adequate for a single table with 2 players.
    *   **Safety**: Eliminates race conditions in game logic.
*   **Alternatives Considered**:
    *   *Thread-per-client*: Scales poorly, requires complex locking for shared Table state.
    *   *Multi-threaded thread pool*: Overkill for a single table; complexity of synchronization outweighs benefits.

### 2. Serialization Protocol
*   **Decision**: JSON (via `nlohmann/json`).
*   **Rationale**:
    *   **Readability**: Human-readable, easy to debug.
    *   **Flexibility**: Schema-less, easy to evolve.
    *   **Standard**: User mandated `nlohmann/json`.
*   **Alternatives Considered**:
    *   *Protobuf*: More efficient, but requires compilation steps and less readable. Overkill for low-frequency poker moves.

### 3. Game State Management
*   **Decision**: Centralized `GameManager` owning `Table` and `Deck`.
*   **Rationale**:
    *   Encapsulates rule enforcement.
    *   Separates networking (Server) from logic (GameManager).
    *   Allows unit testing of game logic without networking.

### 4. Bot Implementation
*   **Decision**: State machine driven by server messages.
*   **Rationale**:
    *   Bot needs to react to `GAME_STATE_UPDATE` or `REQUEST_ACTION`.
    *   Blocking sleeps for "human delay" would block the network loop if single-threaded.
    *   **Solution**: Use `asio::steady_timer` for delays to keep the event loop running.

## Integration Patterns

### WebSocket Message Structure
*   **Format**: `{ "type": "MESSAGE_TYPE", "payload": { ... } }`
*   **Types**:
    *   `LOGIN`: Client -> Server (Identity)
    *   `JOIN`: Client -> Server (Seat request)
    *   `GAME_UPDATE`: Server -> Client (Full state or delta)
    *   `ACTION_REQUEST`: Server -> Client (Your turn)
    *   `ACTION`: Client -> Server (Fold/Call/Bet)
    *   `ERROR`: Server -> Client (Reject)

### Error Handling
*   Server catches exceptions in message handlers and sends `ERROR` message to client.
*   Connection drops handled by `Boost.Beast` disconnect handlers -> triggers "grace period" logic.