# Implementation Plan: Heads Up NLHE Server and Bot Client

**Branch**: `001-heads-up-nlhe` | **Date**: 2025-12-08 | **Spec**: [specs/001-heads-up-nlhe/spec.md](specs/001-heads-up-nlhe/spec.md)
**Input**: Feature specification from `/home/riddler/geminispec/specs/001-heads-up-nlhe/spec.md`

## Summary

Build a Heads-Up No Limit Hold'em (NLHE) server and an autonomous bot client using modern C++. The server will host a single table for 2 players, enforcing standard NLHE rules. The bot client will play with a random strategy, handle automatic top-ups, and simulate human delays. Communication will be via WebSockets using JSON data payloads.

## Technical Context

**Language/Version**: C++20
**Primary Dependencies**: 
- WebSocket Library (NEEDS CLARIFICATION: Boost.Beast vs uWebSockets?)
- JSON Library (NEEDS CLARIFICATION: nlohmann/json vs rapidjson?)
**Storage**: In-memory (Game state persistence not required per spec)
**Testing**: NEEDS CLARIFICATION (Google Test vs Catch2?)
**Target Platform**: Linux
**Project Type**: Client/Server (CLI/Console)
**Performance Goals**: Real-time interaction (<100ms processing), stable socket connections.
**Constraints**: Must handle disconnections and re-connections gracefully (state recovery).

## Constitution Check

*GATE: Passed. (Constitution is generic, adhering to standard C++ best practices).*

## Project Structure

### Documentation (this feature)

```text
specs/001-heads-up-nlhe/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
└── contracts/
    └── game-protocol.md
```

### Source Code (repository root)

```text
CMakeLists.txt
src/
├── common/             # Shared logic (Game rules, Card/Deck models, Protocol definitions)
├── server/             # Game server logic (WebSocket server, Game loop)
└── client/             # Bot client logic (WebSocket client, Decision engine)
tests/
├── unit/               # Unit tests for logic
└── integration/        # Integration tests for server-client flow
```

**Structure Decision**: A single CMake monorepo structure. `common` library allows sharing data structures and serialization logic between `server` and `client`, reducing duplication and ensuring protocol consistency.

## Complexity Tracking

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| N/A       |            |                                     |