# Implementation Plan: Heads Up NLHE Server and Bot Client

**Branch**: `001-heads-up-nlhe` | **Date**: 2025-12-08 | **Spec**: [specs/001-heads-up-nlhe/spec.md](specs/001-heads-up-nlhe/spec.md)
**Input**: Feature specification from `specs/001-heads-up-nlhe/spec.md`

## Summary

Build a C++20 WebSocket server using Boost.Beast to host a single Heads-Up No Limit Hold'em table. Develop a bot client that connects, plays with a random strategy, simulates human delays, and automatically tops up chips. The system must handle disconnects gracefully with configurable timeouts.

## Technical Context

**Language/Version**: C++20
**Primary Dependencies**: Boost.Beast (Network), Boost.Asio (Async I/O), nlohmann/json (JSON)
**Storage**: N/A (In-memory only)
**Testing**: GTest (Google Test) for Unit, Python for Integration
**Target Platform**: Linux
**Project Type**: Server (Console) + Client (Console)
**Performance Goals**: Low latency game loop, stable connection for 2 clients.
**Constraints**: Single threaded event loop (for simplicity and thread-safety).
**Scale/Scope**: 1 Table, 2 Players.

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

*   **I. Clean Code**: Adhering to C++20 standards.
*   **II. Test-First**: Unit tests for Game Logic (Deck, Hand Eval) and Integration tests for Server-Client flow.
*   **III. Consistent UX**: Standard JSON protocol.
*   **IV. Modular Architecture**: Separation of `common` (logic), `server` (network/state), and `client` (bot).

## Project Structure

### Documentation (this feature)

```text
specs/001-heads-up-nlhe/
├── plan.md              # This file
├── research.md          # Technical decisions
├── data-model.md        # Entities and Schema
├── quickstart.md        # Run instructions
├── contracts/           # API/Protocol definitions
│   └── game-protocol.md
└── tasks.md             # Development tasks
```

### Source Code

```text
src/
├── common/             # Shared game logic and types
│   ├── card.hpp
│   ├── types.hpp
│   └── protocol.hpp    # JSON serialization helpers
├── server/             # Game Server
│   ├── server.hpp      # Network handling
│   ├── game.hpp        # Game state machine
│   └── main.cpp
└── client/             # Bot Client
    ├── bot.hpp         # Bot strategy and state
    ├── client.hpp      # Network handling
    └── main.cpp

tests/
├── unit/               # GTest for common/ and server/ logic
└── integration/        # Python/Shell scripts for end-to-end
```

**Structure Decision**: Split into `server` and `client` binaries with a `common` library for shared logic (cards, protocol types).

## Complexity Tracking

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| None | N/A | N/A |