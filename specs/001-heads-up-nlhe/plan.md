# Implementation Plan: Heads Up NLHE Server and Bot Client

**Branch**: `001-heads-up-nlhe` | **Date**: 2025-12-08 | **Spec**: [specs/001-heads-up-nlhe/spec.md](specs/001-heads-up-nlhe/spec.md)
**Input**: Feature specification from `/specs/001-heads-up-nlhe/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

Build a dedicated C++20 game server that hosts a single Heads-Up No Limit Texas Hold'em (NLHE) table for two players. The system includes an autonomous bot client that connects to the server, plays with a random strategy, mimics human delays, and automatically rebuys chips when low. The server must robustly handle player disconnections, reconnection grace periods, and timeouts, ensuring the game state remains valid throughout.

## Technical Context

<!--
  ACTION REQUIRED: Replace the content in this section with the technical details
  for the project. The structure here is presented in advisory capacity to guide
  the iteration process.
-->

**Language/Version**: C++20  
**Primary Dependencies**: Boost.Beast (WebSockets), nlohmann/json (Serialization), Boost.Asio (Networking)  
**Storage**: N/A (In-memory state)  
**Testing**: Google Test (Unit), Pytest (Integration)  
**Target Platform**: Linux  
**Project Type**: Client/Server CLI  
**Performance Goals**: Low latency for game actions, but not high-frequency trading level.  
**Constraints**: Must handle network instability (disconnects) gracefully.  
**Scale/Scope**: 1 server instance = 1 table, 2 max clients.

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- **I. Clean Code & Standards**: C++20 standards will be enforced.
- **II. Test-First Methodology**: Unit tests for game logic (`common`, `server`) and integration tests for network protocol are planned.
- **III. Consistent User Experience**: CLI arguments for configuration (timeouts, ports).
- **IV. Modular Architecture**: Separation of `client`, `server`, and `common` (protocol/logic).

**Gate Status**: PASS

## Project Structure

### Documentation (this feature)

```text
specs/001-heads-up-nlhe/
├── plan.md              # This file
├── research.md          # Technology choices and rationale
├── data-model.md        # Entities and state definitions
├── quickstart.md        # Usage guide
├── contracts/           # API/Protocol definitions
└── tasks.md             # Implementation tasks
```

### Source Code (repository root)
<!--
  ACTION REQUIRED: Replace the placeholder tree below with the concrete layout
  for this feature. Delete unused options and expand the chosen structure with
  real paths (e.g., apps/admin, packages/something). The delivered plan must
  not include Option labels.
-->

```text
src/
├── client/
│   ├── main.cpp
│   ├── network_client.hpp
│   └── bot_state.hpp
├── server/
│   ├── main.cpp
│   ├── server.hpp
│   ├── game_manager.hpp
│   └── player.hpp
└── common/
    ├── protocol.hpp
    ├── card.hpp
    ├── hand_evaluator.hpp
    └── types.hpp

tests/
├── unit/
│   ├── test_card.cpp
│   └── test_hand_evaluator.cpp
└── integration/
    └── test_core_gameplay.py
```

**Structure Decision**: A standard C++ project layout with separated client/server executables and a shared library for common game logic and protocol definitions.

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| N/A | | |
