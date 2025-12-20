# Tech-Spec: Heads-Up Poker Server in Rust

**Created:** 2025-12-12
**Status:** Ready for Development

## Overview

### Problem Statement

Build a production-grade WebSocket-based poker server in Rust that supports heads-up (2-player) No Limit Hold'em cash games. The server needs to handle multiple concurrent tables, manage player accounts, persist game state to a database, and gracefully handle player disconnections.

### Solution

A Rust-based WebSocket server using async I/O (tokio) that:
- Manages up to 5 concurrent heads-up poker tables
- Handles player authentication and session management
- Implements complete Texas Hold'em game logic (deal, betting rounds, showdown)
- Persists player accounts and game state in SQLite
- Communicates via JSON messages over WebSocket
- Supports disconnection/reconnection without losing game state

### Scope (In/Out)

**In Scope - MVP v1:**
- ✅ Player signup/login (username/password)
- ✅ WebSocket connection management
- ✅ Create/join/leave tables (heads-up only)
- ✅ Complete No Limit Hold'em game flow
- ✅ Buy-in: 20-100 BB (big blinds), blinds 0.5/1
- ✅ Play money system (faucet: players can top-up to 100 chips when below 100)
- ✅ Disconnection handling and reconnection
- ✅ Multiple concurrent tables (up to 5)
- ✅ Game state persistence
- ✅ Comprehensive testing (unit, integration, stress)

**Out of Scope - Beyond v1:**
- ❌ Real money transactions
- ❌ Mental poker (cryptographic card dealing)
- ❌ Tournament support
- ❌ Multi-player tables (>2 players)
- ❌ Web/mobile client implementation
- ❌ Rake/house fees
- ❌ Chat functionality
- ❌ Spectator mode

## Context for Development

### Codebase Patterns

This is a **greenfield Rust project**. Follow these patterns:

**Project Structure:**
```
poker-server/
├── src/
│   ├── main.rs              # Entry point, server initialization
│   ├── lib.rs               # Library exports
│   ├── config.rs            # Configuration management
│   ├── models/              # Data models
│   │   ├── mod.rs
│   │   ├── player.rs        # Player account model
│   │   ├── table.rs         # Table state model
│   │   ├── game.rs          # Game state model
│   │   └── card.rs          # Card/deck models
│   ├── db/                  # Database layer
│   │   ├── mod.rs
│   │   └── sqlite.rs        # SQLite persistence
│   ├── game_logic/          # Poker game engine
│   │   ├── mod.rs
│   │   ├── hand_evaluator.rs
│   │   ├── dealer.rs
│   │   └── betting.rs
│   ├── websocket/           # WebSocket handling
│   │   ├── mod.rs
│   │   ├── server.rs
│   │   ├── session.rs
│   │   └── messages.rs      # JSON message types
│   └── table_manager/       # Table orchestration
│       ├── mod.rs
│       └── manager.rs
├── tests/
│   ├── integration/
│   └── load/
├── Cargo.toml
└── README.md
```

**Rust Patterns:**
- Use `async/await` with `tokio` runtime
- Error handling: `Result<T, E>` with custom error types using `thiserror` crate
- Use `Arc<RwLock<State>>` for shared table state (better read scalability than Mutex)
- Message queue per table to guarantee WebSocket action ordering
- Prefer `serde` for JSON serialization
- Use SQLx for type-safe SQL with connection pooling (`sqlx::Pool`)

**Database Schema (SQLite):**
```sql
CREATE TABLE players (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    chips INTEGER DEFAULT 100,  -- Start with 100 play money chips
    hands_played INTEGER DEFAULT 0,
    hands_won INTEGER DEFAULT 0,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX idx_players_username ON players(username);

CREATE TABLE tables (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    small_blind REAL NOT NULL,
    big_blind REAL NOT NULL,
    max_players INTEGER DEFAULT 2,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE game_sessions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    table_id INTEGER NOT NULL,
    started_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    ended_at DATETIME,
    FOREIGN KEY (table_id) REFERENCES tables(id)
);
```

### Files to Reference

**External Resources:**
- Rust Tokio: https://tokio.rs/
- WebSocket: `tokio-tungstenite` crate
- Database: `sqlx` with SQLite driver
- Poker hand evaluation: Consider `poker` crate or implement custom evaluator
- Password hashing: `argon2` crate

### Technical Decisions

1. **WebSocket Library**: Use `tokio-tungstenite` for async WebSocket support
2. **JSON**: Use `serde_json` for message serialization
3. **Database**: SQLite with `sqlx` for type-safe queries
4. **Authentication**: Simple username/password with Argon2 hashing (no JWT needed for v1)
5. **Concurrency**: Max 5 concurrent tables initially
6. **Blinds**: Fixed at 0.5/1 for all tables in MVP
7. **Buy-in Range**: 20-100 BB (20-100 chips with BB=1)

## Implementation Plan

### Tasks

#### Phase 1: Project Setup & Foundation
- [ ] **Task 1.1**: Initialize Rust project with Cargo
  - Add dependencies: tokio, tokio-tungstenite, serde, serde_json, sqlx, argon2, thiserror, anyhow
  - Set up basic project structure
  - Configure SQLite database connection with `sqlx::Pool`
  - Define custom error types using `thiserror` (NetworkError, GameError, DbError)

- [ ] **Task 1.2**: Implement data models
  - Card and Deck structs
  - Player model with authentication
  - Table and GameState models
  - JSON message types (LoginRequest, JoinTableRequest, BetAction, etc.)

- [ ] **Task 1.3**: Set up database schema and migrations
  - Create SQLite schema (players, tables, game_sessions)
  - Implement player CRUD operations
  - Test database persistence

#### Phase 2: Game Logic Engine
- [ ] **Task 2.1**: Implement poker hand evaluator
  - Card comparison logic
  - Hand ranking (high card → royal flush)
  - Winner determination
  - Unit tests for all hand types

- [ ] **Task 2.2**: Implement dealer logic
  - Deck shuffling with cryptographic randomness
  - Deal hole cards
  - Deal flop, turn, river
  - Pot management (heads-up only, no side pots needed)

- [ ] **Task 2.3**: Implement betting rounds
  - Action validation (fold, check, call, raise)
  - Bet sizing rules (min/max raise)
  - All-in handling (no side pots for heads-up)

- [ ] **Task 2.4**: Game state machine
  - Pre-flop → Flop → Turn → River → Showdown
  - Automatic blind posting
  - Button/blinds rotation
  - Win condition detection

#### Phase 3: WebSocket Server
- [ ] **Task 3.1**: WebSocket server setup
  - Create WebSocket listener on port (e.g., 8080)
  - Handle connection/disconnection events
  - Message routing (text → JSON parsing with validation)
  - Implement message queue per table for action ordering guarantees

- [ ] **Task 3.2**: Session management
  - Track active WebSocket connections
  - Associate sessions with players
  - Handle authentication via WebSocket
  - Implement heartbeat/ping-pong

- [ ] **Task 3.3**: Message handlers
  - Login/Signup handlers
  - CreateTable/JoinTable/LeaveTable
  - GameAction (Fold, Check, Call, Raise)
  - Broadcast game state updates to players

#### Phase 4: Table Manager
- [ ] **Task 4.1**: Table orchestration
  - Create/destroy tables dynamically
  - Enforce 5 concurrent table limit
  - Seat players at tables
  - Start games when 2 players seated

- [ ] **Task 4.2**: Disconnection handling
  - Detect player disconnects
  - Pause game state with configurable grace period (default 30s)
  - Auto-fold if timeout expires
  - Reconnection logic (restore full game state)

- [ ] **Task 4.3**: Buy-in and chip management
  - Validate buy-in range (20-100 BB)
  - Deduct chips on buy-in
  - Implement play money faucet: allow top-up to 100 when chips < 100
  - Add `TopUpRequest`/`TopUpResponse` WebSocket messages
  - Update player balances and statistics in DB (chips, hands_played, hands_won)

#### Phase 5: Testing & Hardening
- [ ] **Task 5.1**: Unit tests
  - Test hand evaluator thoroughly (all 10 hand types + tie-breakers)
  - Property-based tests for hand evaluator using `proptest`
  - Test betting validation (min/max raise, all-in edge cases)
  - Test game state transitions
  - Test database operations with fixtures
  - Test error type conversions and propagation

- [ ] **Task 5.2**: Integration tests
  - End-to-end game flow (2 players, full hand to showdown)
  - Multi-table scenarios (5 concurrent games)
  - Disconnection/reconnection with grace period
  - Top-up edge cases (0 chips, 99.5 chips, concurrent top-up attempts)
  - Edge cases (all-in calculations, tie resolution)
  - CI/CD: GitHub Actions workflow for automated test execution

- [ ] **Task 5.3**: Load/stress tests
  - 5 concurrent tables with active games (100 hands per table)
  - Rapid connect/disconnect cycles
  - Reconnection storm test (10 players disconnect/reconnect simultaneously)
  - Message flooding protection
  - Memory profiling over 1000+ hands to detect leaks
  - Measure: latency, throughput, memory usage

- [ ] **Task 5.4**: Security hardening
  - Input validation (sanitize all JSON inputs)
  - Fuzzing for JSON message parsing (critical attack vector)
  - Rate limiting on authentication endpoints
  - SQL injection prevention (parameterized queries with sqlx)
  - Password policy enforcement (min 8 chars, complexity rules)

### Acceptance Criteria

**AC1: Player Authentication**
- Given a new player
- When they send a SignupRequest with username/password
- Then their account is created with hashed password and 0 chips

**AC2: Table Creation and Joining**
- Given a logged-in player
- When they create a table
- Then a new table is created with 0.5/1 blinds
- And the player is seated

**AC3: Game Start with 2 Players**
- Given a table with 2 players
- When both players have bought in (20-100 BB)
- Then the game starts automatically
- And hole cards are dealt
- And blinds are posted

**AC4: Complete Hand Flow**
- Given a game in progress
- When players complete all betting rounds (pre-flop, flop, turn, river)
- Then hands are evaluated at showdown
- And the winner receives the pot
- And chips are updated in the database

**AC5: Disconnection Handling**
- Given a player in an active game
- When their WebSocket disconnects
- Then the game pauses for 30 seconds
- And if they reconnect, they rejoin the game
- And if they don't, they auto-fold and lose their chips in the pot

**AC6: Multiple Concurrent Tables**
- Given the server is running
- When 5 tables are created
- Then all 5 games run independently
- And players can join any available table

**AC7: All-In Scenario**
- Given a player with fewer chips than the current bet
- When they go all-in
- Then the pot is calculated correctly (no side pots for heads-up)
- And the showdown resolves properly

**AC8: Play Money Top-Up**
- Given a player with chips < 100
- When they request a top-up
- Then their chips are set to 100
- And the database is updated
- And if chips >= 100, top-up is rejected

**AC9: Player Statistics**
- Given a completed hand
- When the winner is determined
- Then `hands_played` increments for both players
- And `hands_won` increments for the winner
- And statistics are persisted to database

## Additional Context

### Dependencies

**Cargo.toml (suggested crates):**
```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
tokio-tungstenite = "0.21"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
sqlx = { version = "0.7", features = ["runtime-tokio-rustls", "sqlite"] }
argon2 = "0.5"
rand = "0.8"
anyhow = "1"
thiserror = "1"

[dev-dependencies]
criterion = "0.5"  # For benchmarks
proptest = "1"     # Property-based testing
```

### Testing Strategy

**Unit Tests:**
- Hand evaluator: All 10 hand types with tie-breakers
- Property-based tests: Generate random hands and verify evaluation consistency
- Betting logic: Min/max raise, all-in edge cases
- Deck: Shuffle randomness, no duplicate cards
- Models: Serialization/deserialization of all JSON messages
- Error handling: Test all error type conversions

**Integration Tests:**
- Full game simulation (2 bots playing to showdown)
- Database persistence (create player → buy-in → game → verify chips + stats updated)
- WebSocket flow (connect → auth → join → play → disconnect → reconnect)
- Top-up flow (deplete chips → top-up → verify balance = 100)
- CI/CD: Automated test suite in GitHub Actions

**Load Tests:**
- 5 tables, 10 players, 100 hands per table
- Reconnection storm: 10 simultaneous disconnects/reconnects
- Measure: latency, throughput, memory usage over 1000+ hands
- Tool: Custom load test harness or `criterion` for benchmarks

**Security Tests:**
- Fuzz testing on JSON message parsing (use `cargo-fuzz`)
- SQL injection attempts (verify parameterized queries work)
- Brute force login attempts (rate limit validation)
- Malformed WebSocket message handling

### Notes

**Performance Considerations:**
- Use `Arc<RwLock<T>>` for shared table state (better read scalability than Mutex)
- Implement per-table message queue to guarantee action ordering
- Avoid blocking operations in async context
- Pre-allocate deck vectors to reduce allocations
- Use `sqlx::Pool` for database connection pooling

**Future Extensibility:**
- Design game engine to support >2 players (even if MVP limits to 2)
- Keep mental poker in mind (abstract card dealing into trait)
- Database schema supports multiple game types (add game_type column later)
- Player statistics foundation enables leaderboards and analytics in v2

**Deployment:**
- Target: Linux VPS (Ubuntu/Debian)
- Run as systemd service
- SQLite file stored in `/var/lib/poker-server/poker.db`
- Expose WebSocket on port 8080 (reverse proxy with nginx optional)

**Development Workflow:**
1. Implement each phase sequentially
2. Write tests alongside code (TDD recommended for game logic)
3. Use `cargo clippy` for linting
4. Use `cargo fmt` for formatting
5. CI/CD: GitHub Actions for automated testing (optional but recommended)
