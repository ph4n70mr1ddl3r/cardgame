# Tasks: Heads Up NLHE Server and Bot Client

**Input**: Design documents from `specs/001-heads-up-nlhe/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/

**Tests**: Tests are included as foundational and integration steps where appropriate, using Google Test as per plan.

**Organization**: Tasks are grouped by user story, with US1 (Core) and US3 (Disconnects) being Priority 1, followed by US2 (Top-Up).

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel
- **[Story]**: User Story label (US1, US2, US3)
- Path convention: `src/common/`, `src/server/`, `src/client/`

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization, build system, and dependencies.

- [ ] T001 Create project directory structure (`src/common`, `src/server`, `src/client`, `tests/unit`, `tests/integration`)
- [ ] T002 Create root `CMakeLists.txt` with C++20 standard and compiler warnings
- [ ] T003 Configure `CMakeLists.txt` to fetch dependencies: Boost (Beast/Asio), nlohmann/json, GoogleTest
- [ ] T004 [P] Create `tests/CMakeLists.txt` and `src/CMakeLists.txt` sub-project definitions
- [ ] T005 [P] Setup `.gitignore` for C++ build artifacts

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core domain entities and shared protocol definitions required by both Server and Client.

**⚠️ CRITICAL**: Must be complete before User Stories.

- [ ] T006 Create `src/common/types.hpp` with Enums (Suit, Rank, GameState, PlayerStatus, ActionType)
- [ ] T007 [P] Implement `src/common/card.hpp` and `src/common/card.cpp` (Card struct, Deck class with shuffle/draw)
- [ ] T008 [P] Create `tests/unit/test_card.cpp` and implement unit tests for Deck shuffling and drawing
- [ ] T009 Implement `src/common/protocol.hpp` defining JSON message structures (Login, Action, GameState, etc.) using `nlohmann/json`
- [ ] T010 [P] Implement `src/common/hand_evaluator.hpp` and `src/common/hand_evaluator.cpp` for basic NLHE hand ranking
- [ ] T011 [P] Create `tests/unit/test_hand_evaluator.cpp` to verify hand ranking logic

**Checkpoint**: Shared library builds, unit tests for logic pass.

---

## Phase 3: User Story 1 - Core Gameplay & Bot Logic (Priority: P1) 🎯 MVP

**Goal**: A functional server hosting a Heads-Up game and autonomous bots playing hands.

**Independent Test**: Connect 2 bots, verify they play through hands (Deal -> Bet -> Showdown) without crashing.

### Implementation for User Story 1

- [ ] T012 [US1] Create `src/server/player.hpp` to track socket session, stack, hole cards, and status
- [ ] T013 [US1] Create `src/server/table.hpp` and `src/server/table.cpp` managing GameState, Pot, and Deck
- [ ] T014 [US1] Implement `src/server/game_loop.cpp` handling state transitions (Preflop -> Flop -> Turn -> River -> Showdown)
- [ ] T015 [US1] Implement `src/server/server.cpp` using Boost.Beast to accept WebSocket connections and route messages
- [ ] T016 [US1] Integrate `src/server/main.cpp` to start the server and game loop
- [ ] T017 [P] [US1] Create `src/client/bot_state.hpp` to track client-side game view
- [ ] T018 [P] [US1] Implement `src/client/strategy.cpp` for random valid action selection
- [ ] T019 [US1] Implement `src/client/network_client.cpp` using Boost.Beast to connect and handle messages
- [ ] T020 [US1] Implement `src/client/main.cpp` with random delay loop (FR-005)
- [ ] T021 [P] [US1] Create `tests/integration/test_core_gameplay.py` (or C++ equivalent) to spawn server and 2 bots and assert exit code/logs

**Checkpoint**: Core game loop functional. Bots can play indefinitely (until bust).

---

## Phase 4: User Story 3 - Disconnection & Timeout Management (Priority: P1)

**Goal**: Robust handling of player dropouts and turn timeouts.

**Independent Test**: Kill a bot process during a hand; Server should wait grace period, then fold/sit-out player.

### Implementation for User Story 3

- [ ] T022 [US3] Update `src/server/server.cpp` to detect WebSocket disconnection events
- [ ] T023 [US3] Implement `src/server/timeout_manager.hpp` using Boost.Asio timers for turn limits and grace periods
- [ ] T024 [US3] Update `src/server/table.cpp` to handle `DISCONNECTED` state and trigger "sit out" (FR-008)
- [ ] T025 [US3] Implement logic in `src/server/game_loop.cpp` to fold players who timeout or disconnect
- [ ] T026 [US3] Implement player removal logic (FR-009) after extended "sit out" duration
- [ ] T027 [US3] Update `src/client/network_client.cpp` to attempt reconnection on connection loss
- [ ] T028 [US3] Update `src/server/server.cpp` to handle `LOGIN` from a reconnecting player (restore session)

**Checkpoint**: Server survives client crashes. Reconnection restores state.

---

## Phase 5: User Story 2 - Automatic Stack Top-Up (Priority: P2)

**Goal**: Bots automatically rebuy when low on chips to keep the simulation running.

**Independent Test**: Manually set bot stack < 5BB, verify it requests top-up and stack resets to 100BB.

### Implementation for User Story 2

- [ ] T029 [US2] Update `src/common/protocol.hpp` to ensure `TOP_UP` message is defined
- [ ] T030 [US2] Update `src/server/game_loop.cpp` to process `TOP_UP` messages (FR-006)
- [ ] T031 [US2] Add logic in `src/server/table.cpp` to validate top-up (only between hands or when allowed)
- [ ] T032 [US2] Update `src/client/bot_logic.cpp` to check stack size at Hand End
- [ ] T033 [US2] Implement logic to send `TOP_UP` request if stack < 5BB (FR-006)

**Checkpoint**: Bots never run out of chips permanently.

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Final cleanup and non-functional requirements.

- [ ] T034 [P] Add detailed logging to `src/server/logger.hpp` (Game history, errors)
- [ ] T035 [P] Update `src/client/main.cpp` to parse command line args (host, port, bot name)
- [ ] T036 Review `specs/001-heads-up-nlhe/quickstart.md` and verify instructions work
- [ ] T037 Ensure clean shutdown handling in `src/server/main.cpp` (SIGINT handler)

---

## Dependencies & Execution Order

### Phase Dependencies
1. **Phase 1 & 2** (Setup/Foundation) must be 100% complete before any User Story.
2. **Phase 3** (US1 - Core) depends on Foundation.
3. **Phase 4** (US3 - Disconnects) depends on Phase 3 (needs working game loop to interrupt).
4. **Phase 5** (US2 - Top-Up) depends on Phase 3 (needs working game loop). Can be done in parallel with Phase 4 if resources allow, but P2 priority suggests doing it after P1s.

### Parallel Opportunities
- **Foundation**: `Card`/`Deck` (T007) and `HandEvaluator` (T010) are independent.
- **US1**: Server (`T012`-`T016`) and Client (`T017`-`T020`) can be developed in parallel once Protocol (`T009`) is agreed upon.
- **Testing**: Integration tests (`T021`) can be written while implementation proceeds.

## Implementation Strategy

### MVP (US1 + US3)
Focus on getting a stable game loop that doesn't crash when a bot disconnects. The Top-Up feature (US2) is a convenience for long-running simulations and can be added last.

1. Build Shared Lib & Protocol
2. Build Basic Server (Accept connections)
3. Build Basic Bot (Connect & Login)
4. Implement Game Logic (Deal, Bet, Win)
5. Add Timeout/Disconnect handling
6. Add Top-Up Logic
