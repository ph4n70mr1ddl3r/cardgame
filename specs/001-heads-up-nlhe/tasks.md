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

- [x] T001 Create project directory structure (`src/common`, `src/server`, `src/client`, `tests/unit`, `tests/integration`)
- [x] T002 Create root `CMakeLists.txt` with C++20 standard and compiler warnings
- [x] T003 Configure `CMakeLists.txt` to fetch dependencies: Boost (Beast/Asio), nlohmann/json, GoogleTest
- [x] T004 [P] Create `tests/CMakeLists.txt` and `src/CMakeLists.txt` sub-project definitions
- [x] T005 [P] Setup `.gitignore` for C++ build artifacts

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core domain entities and shared protocol definitions required by both Server and Client.

**⚠️ CRITICAL**: Must be complete before User Stories.

- [x] T006 Create `src/common/types.hpp` with Enums (Suit, Rank, GameState, PlayerStatus, ActionType)
- [x] T007 [P] Implement `src/common/card.hpp` and `src/common/card.cpp` (Card struct, Deck class with shuffle/draw)
- [x] T008 [P] Create `tests/unit/test_card.cpp` and implement unit tests for Deck shuffling and drawing
- [x] T009 Implement `src/common/protocol.hpp` defining JSON message structures (Login, Action, GameState, etc.) using `nlohmann/json`
- [x] T010 [P] Implement `src/common/hand_evaluator.hpp` and `src/common/hand_evaluator.cpp` for basic NLHE hand ranking
- [x] T011 [P] Create `tests/unit/test_hand_evaluator.cpp` to verify hand ranking logic

**Checkpoint**: Shared library builds, unit tests for logic pass.

---

## Phase 3: User Story 1 - Core Gameplay & Bot Logic (Priority: P1) 🎯 MVP

**Goal**: A functional server hosting a Heads-Up game and autonomous bots playing hands.

**Independent Test**: Connect 2 bots, verify they play through hands (Deal -> Bet -> Showdown) without crashing.

### Implementation for User Story 1

- [x] T012 [US1] Create `src/server/player.hpp` to track socket session, stack, hole cards, and status
- [x] T013 [US1] Create `src/server/table.hpp` and `src/server/table.cpp` managing GameState, Pot, and Deck
- [x] T014 [US1] Implement `src/server/game_loop.cpp` (implemented as `game_manager.cpp`) handling state transitions
- [x] T015 [US1] Implement `src/server/server.cpp` using Boost.Beast to accept WebSocket connections and route messages
- [x] T016 [US1] Integrate `src/server/main.cpp` to start the server and game loop
- [x] T017 [P] [US1] Create `src/client/bot_state.hpp` to track client-side game view
- [x] T018 [P] [US1] Implement `src/client/strategy.cpp` (as header) for random valid action selection
- [x] T019 [US1] Implement `src/client/network_client.cpp` using Boost.Beast to connect and handle messages
- [x] T020 [US1] Implement `src/client/main.cpp` with random delay loop (FR-005)
- [x] T021 [P] [US1] Create `tests/integration/test_core_gameplay.py` (or C++ equivalent) to spawn server and 2 bots and assert exit code/logs

**Checkpoint**: Core game loop functional. Bots can play indefinitely (until bust).

---

## Phase 4: User Story 3 - Disconnection & Timeout Management (Priority: P1)

**Goal**: Robust handling of player dropouts and turn timeouts.

**Independent Test**: Kill a bot process during a hand; Server should wait grace period, then fold/sit-out player.

### Implementation for User Story 3

- [x] T022 [US3] Update `src/server/server.cpp` to detect WebSocket disconnection events (handled in initial impl)
- [x] T023 [US3] Implement `src/server/timeout_manager.hpp` using Boost.Asio timers for turn limits and grace periods
- [x] T024 [US3] Update `src/server/table.cpp` to handle `DISCONNECTED` state and trigger "sit out" (FR-008)
- [x] T025 [US3] Implement logic in `src/server/game_loop.cpp` to fold players who timeout or disconnect
- [x] T026 [US3] Implement player removal logic (FR-009) after extended "sit out" duration
- [x] T027 [US3] Update `src/client/network_client.cpp` to attempt reconnection on connection loss
- [x] T028 [US3] Update `src/server/server.cpp` to handle `LOGIN` from a reconnecting player (restore session)

**Checkpoint**: Server survives client crashes. Reconnection restores state.

---

## Phase 5: User Story 2 - Automatic Stack Top-Up (Priority: P2)

**Goal**: Bots automatically rebuy when low on chips to keep the simulation running.

**Independent Test**: Manually set bot stack < 5BB, verify it requests top-up and stack resets to 100BB.

### Implementation for User Story 2

- [x] T029 [US2] Update `src/common/protocol.hpp` to ensure `TOP_UP` message is defined
- [x] T030 [US2] Update `src/server/game_loop.cpp` to process `TOP_UP` messages (FR-006)
- [x] T031 [US2] Add logic in `src/server/table.cpp` to validate top-up (only between hands or when allowed)
- [x] T032 [US2] Update `src/client/bot_logic.cpp` to check stack size at Hand End
- [x] T033 [US2] Implement logic to send `TOP_UP` request if stack < 5BB (FR-006)

**Checkpoint**: Bots never run out of chips permanently.

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Final cleanup and non-functional requirements.

- [x] T034 [P] Add detailed logging to `src/server/logger.hpp` (Game history, errors)
- [x] T035 [P] Update `src/client/main.cpp` to parse command line args (host, port, bot name) (Done in initial impl)
- [x] T036 Review `specs/001-heads-up-nlhe/quickstart.md` and verify instructions work
- [x] T037 Ensure clean shutdown handling in `src/server/main.cpp` (SIGINT handler)