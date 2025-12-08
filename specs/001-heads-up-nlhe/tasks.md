# Tasks: Heads Up NLHE Server and Bot Client

**Feature**: `001-heads-up-nlhe`
**Spec**: `specs/001-heads-up-nlhe/spec.md`
**Plan**: `specs/001-heads-up-nlhe/plan.md`
**Status**: Pending

## Phase 1: Setup
**Goal**: Initialize project structure and build system for C++20 with Boost.Beast.

- [ ] T001 Create root CMakeLists.txt with C++20 standard and Boost dependencies in `CMakeLists.txt`
- [ ] T002 Setup common library structure and CMake in `src/common/CMakeLists.txt`
- [ ] T003 Setup server executable structure and CMake in `src/server/CMakeLists.txt`
- [ ] T004 Setup client executable structure and CMake in `src/client/CMakeLists.txt`
- [ ] T005 Setup unit test infrastructure with GoogleTest in `tests/unit/CMakeLists.txt`

## Phase 2: Foundation (Shared Logic)
**Goal**: Implement core poker logic and protocol definitions used by both server and client.
**Blocking**: Must be completed before User Stories.

- [ ] T006 [P] Implement Card class (Rank, Suit, parsing) in `src/common/card.hpp` and `src/common/card.cpp`
- [ ] T007 [P] Implement Deck class (shuffling, dealing) in `src/common/deck.hpp` and `src/common/deck.cpp`
- [ ] T008 [P] Implement HandEvaluator class (7-card evaluation) in `src/common/hand_evaluator.hpp` and `src/common/hand_evaluator.cpp`
- [ ] T009 [P] Define Protocol structs (Message, Payload) and JSON serializers in `src/common/protocol.hpp`
- [ ] T010 Create unit tests for Card and HandEvaluator in `tests/unit/test_card.cpp` and `tests/unit/test_hand_evaluator.cpp`

## Phase 3: User Story 1 - Core Gameplay & Bot Logic
**Goal**: Server hosts a game, 2 bots connect and play valid hands.
**Priority**: P1

- [ ] T011 [US1] Implement Player class (Stack, Status, Hole Cards) in `src/server/player.hpp`
- [ ] T012 [US1] Implement Table class (Pot, Board, Player management) in `src/server/table.hpp` and `src/server/table.cpp`
- [ ] T013 [US1] Implement GameManager State Machine (PreFlop, Flop, etc.) in `src/server/game_manager.hpp` and `src/server/game_manager.cpp`
- [ ] T014 [US1] Implement Session class (WebSocket read/write) in `src/server/session.hpp`
- [ ] T015 [US1] Implement Server class (Acceptor, Room management) in `src/server/server.hpp` and `src/server/server.cpp`
- [ ] T016 [US1] Implement NetworkClient class (WebSocket connection) in `src/client/network_client.hpp` and `src/client/network_client.cpp`
- [ ] T017 [US1] Implement BotState class (Random strategy, Action generation) in `src/client/bot_state.hpp`
- [ ] T018 [US1] Implement Client main loop (Connect, Login, Event Loop) in `src/client/main.cpp`
- [ ] T019 [US1] Wire up Game Logic to Server Messages (Protocol handling) in `src/server/server.cpp`
- [ ] T020 [US1] Create integration test script for full game loop in `tests/integration/test_core_gameplay.py`

## Phase 4: User Story 3 - Disconnection & Timeouts
**Goal**: Handle player disconnects, grace periods, and removal.
**Priority**: P1

- [ ] T021 [US3] Implement TimeoutManager for tracking last activity in `src/server/timeout_manager.hpp`
- [ ] T022 [US3] Add disconnection detection and status update in `src/server/game_manager.cpp`
- [ ] T023 [US3] Implement Reconnection logic (match player_id to existing seat) in `src/server/game_manager.cpp`
- [ ] T024 [US3] Implement "Sit Out" and Player Removal logic after timeouts in `src/server/table.cpp`
- [ ] T025 [US3] Verify disconnect handling with integration test in `tests/integration/test_disconnect.py`

## Phase 5: User Story 2 - Automatic Stack Top-Up
**Goal**: Bots automatically rebuy when stack is low.
**Priority**: P2

- [ ] T026 [US2] Update BotState to check stack < 5BB and send top-up request in `src/client/bot_state.hpp`
- [ ] T027 [US2] Implement top-up request handling in `src/server/game_manager.cpp`
- [ ] T028 [US2] Add unit test for top-up logic in `tests/unit/test_game_manager.cpp`

## Phase 6: Polish
**Goal**: structured logging, configuration, and final verification.

- [ ] T029 Implement structured JSON logger in `src/server/logger.hpp`
- [ ] T030 Add command-line argument parsing (timeouts, ports) in `src/server/main.cpp`
- [ ] T031 Final integration test suite run in `tests/run_all.sh`

## Implementation Strategy
- **MVP (Phase 3)**: Focus on getting two bots to play a full hand (Deal -> Bet -> Showdown) without crashing.
- **Robustness (Phase 4)**: Once gameplay works, add the "happy path" breakers (disconnects).
- **Features (Phase 5)**: Add the auto-rebuy feature last as it's an enhancement to the core loop.

## Dependencies
- US1 (Core) depends on Foundation (Cards/Protocol)
- US3 (Disconnects) depends on US1 (need a running game to disconnect from)
- US2 (Top-Up) depends on US1 (need a stack to deplete)

## Parallel Execution
- **Phase 2**: T006 (Card), T007 (Deck), T008 (Evaluator) can be built in parallel.
- **Phase 3**: Client (T016-T018) and Server (T011-T015) can be developed somewhat independently if Protocol (T009) is solid.
