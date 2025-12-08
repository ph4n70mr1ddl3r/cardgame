# Feature Specification: Heads Up NLHE Server and Bot Client

**Feature Branch**: `001-heads-up-nlhe`  
**Created**: 2025-12-08  
**Status**: Draft  
**Input**: User description: "Build a server that will host 2 clients that will play heads up nlhe. It will use the standard NLHE rules. Handle timeouts gracefully. Build a client that will connect to the server and will start with 100BB. It will be a bot playing with a random strategy. If the stack goes below 5BB, it will automatically top up to 100BB. It will have random delay like a human player. If the client is disconnected, it will be given ample time to return, otherwise it will be considered folded and sat out. After a while it will be removed from the table if it does not return. The server must handle everything gracefully. The server will only accommodate 2 players. So it basically has only 1 table."

## Clarifications

### Session 2025-12-08
- Q: C++ Dependencies for WebSockets and JSON? → A: Boost.Beast + nlohmann/json.
- Q: Player Identity & Reconnection Security? → A: Simple String ID.
- Q: Timeout Configuration? → A: Command-line arguments.
- Q: Preferred logging strategy for server operational activities? → A: Structured logging (JSON) to stdout/stderr.
- Q: Explicitly out-of-scope functionalities/features? → A: GUI, advanced AI, multiple tables, persistent storage.
- Q: Security for player identity and reconnection to prevent impersonation? → A: No additional security measures beyond `player_id` for this initial version.
- Q: How server communicates errors/rejections to client? → A: Standardized JSON error responses with specific error codes/messages.
- Q: Should the server implement rate limiting for client actions? → A: Basic rate limiting (N actions per second) per client.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Core Gameplay & Bot Logic (Priority: P1)

As a user, I want a server that hosts a Heads-Up No Limit Hold'em game and a bot client that can play against it, so that I can simulate poker matches.

**Why this priority**: This is the core functionality. Without the server and playing clients, no other features matter.

**Independent Test**: Can be fully tested by starting the server and connecting two bot clients. They should deal cards, make random moves (bet/fold/check) with delays, and complete hands according to NLHE rules.

**Acceptance Scenarios**:

1. **Given** the server is running, **When** two clients connect, **Then** the game starts automatically with both players having 100BB stacks.
2. **Given** a hand is in progress, **When** it is a bot's turn, **Then** it waits a random duration (simulating human delay) before making a valid random move (check, call, bet, raise, or fold).
3. **Given** the hand reaches showdown or all players fold, **Then** the pot is awarded correctly according to standard NLHE rules and the next hand begins.

---

### User Story 2 - Automatic Stack Top-Up (Priority: P2)

As a bot operator, I want my bot to automatically rebuy chips when it runs low, so that the game can continue indefinitely without manual intervention.

**Why this priority**: Ensures continuous play without the game ending due to bust-outs.

**Independent Test**: Can be tested by forcing a bot to lose chips (or manually setting stack size) and verifying it tops up.

**Acceptance Scenarios**:

1. **Given** a bot's stack is below 5BB at the end of a hand, **When** the next hand is about to start, **Then** the bot automatically tops up its stack to 100BB.
2. **Given** a bot's stack is 5BB or more, **When** the next hand starts, **Then** the stack remains unchanged (no top-up).

---

### User Story 3 - Disconnection & Timeout Management (Priority: P1)

As a system administrator, I want the server to handle player disconnections and timeouts gracefully, so that the game state remains valid and the table isn't blocked by inactive players.

**Why this priority**: Critical for stability and robustness. A single disconnect shouldn't crash the server or freeze the game forever.

**Independent Test**: Can be tested by manually killing a client process or disconnecting its network during a hand.

**Acceptance Scenarios**:

1. **Given** a player disconnects during a hand, **When** it is their turn, **Then** the server provides a reconnection grace period (e.g., 30-60s) before timing out.
2. **Given** a disconnected player does not return within the grace period, **When** the timer expires, **Then** their hand is folded, they are marked as "sitting out," and the game continues.
3. **Given** a player remains disconnected/sitting out for an extended period (e.g., 5 mins), **When** the removal timer expires, **Then** the player is removed from the table entirely.
4. **Given** a player reconnects before being removed, **When** they rejoin, **Then** they are seated back at the table (waiting for next hand if necessary).

### Edge Cases

- What happens when both players disconnect simultaneously? (Server should pause or reset table eventually).
- How does the system handle split pots? (Standard rules apply).
- What happens if a player tries to connect to a full table? (Connection rejected/queued).
- What if a bot tries to bet more than its stack? (Server limits action to All-in).

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The server MUST host exactly one table supporting a maximum of 2 players (Heads-Up).
- **FR-002**: The server MUST enforce standard No Limit Texas Hold'em (NLHE) rules for betting, hand ranking, and pot distribution.
- **FR-003**: The client MUST be an autonomous bot that starts with a stack of 100 Big Blinds (BB).
- **FR-004**: The bot client MUST play with a random strategy (choosing valid actions randomly).
- **FR-005**: The bot client MUST introduce a random delay (e.g., 1-5 seconds) before every action to simulate human timing.
- **FR-006**: The bot client MUST automatically request a top-up to 100BB if its stack falls below 5BB between hands.
- **FR-007**: The server MUST implement a "grace period" (configurable, default ~30-60s) for disconnected players to reconnect before folding their hand.
- **FR-008**: The server MUST set a player to "sit out" status if they fail to reconnect or act within the timeout period.
- **FR-009**: The server MUST remove a player from the table if they remain in "sit out" or disconnected state for an extended duration (default ~5 mins).
- **FR-010**: The server MUST handle client messages gracefully, ignoring invalid or out-of-turn actions without crashing.
- **FR-011**: The server MUST accept reconnection grace period and player removal duration as command-line arguments (with specified defaults).
- **FR-012**: The server MUST communicate errors, rejections, and other status messages to clients using standardized JSON responses, including specific error codes and descriptive messages.
- **FR-013**: The server MUST implement basic rate limiting (e.g., N actions per second) per client to prevent abuse and performance degradation.

### Non-Functional Quality Attributes

- **Observability**: The server MUST implement structured logging (e.g., JSON format) to stdout/stderr for all operational activities, including game state changes, player actions, and errors, to facilitate external log aggregation and analysis.

### Technology Stack

- **Language**: C++20
- **Networking**: Boost.Beast (WebSocket)
- **Serialization**: nlohmann/json
- **Build System**: CMake

### Assumptions

- **Standard Rules**: "Standard NLHE rules" implies Texas Hold'em with standard hand rankings, Small Blind/Big Blind structure, and No-Limit betting.
- **Timing Defaults**: "Ample time" for reconnection is assumed to be 30-60 seconds. "After a while" for removal is assumed to be ~5 minutes. "Random delay" is assumed to be 1-5 seconds.
- **Network**: The server acts as the source of truth; network latency is not explicitly simulated beyond the intentional bot delays.
- **Player Identity Trust**: For reconnection, the server trusts the `player_id` provided by the client in the `LOGIN` message for identity.

## Out-of-Scope

- **Graphical User Interface (GUI)**: No graphical interface for the server or client is planned for this initial iteration.
- **Advanced AI**: The bot client will use a random strategy; no sophisticated poker AI is included.
- **Multiple Tables/Games**: The server is designed to host a single Heads-Up table.
- **Persistent Storage**: Game state, hand histories, or player data will not be persisted across server restarts.

### Key Entities *(include if feature involves data)*

- **Table**: Represents the single game instance, including deck state, pot size, community cards, and active players.
- **Player**: Represents a connected client, tracking their current stack, hole cards, status (active, disconnected, sitting out), and position.
- **HandHistory**: (Implicit) The sequence of actions in a current hand to ensure rule enforcement.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Two bot clients can play 100 consecutive hands without server crash or rule violation.
- **SC-002**: A disconnected client can reconnect within 30 seconds and resume play without game state corruption.
- **SC-003**: Bots consistently maintain a stack >5BB (due to auto top-up) over a 50-hand simulation where they lose chips.
- **SC-004**: Server rejects a 3rd client connection attempt when 2 players are already seated.