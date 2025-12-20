# Poker Server - Heads-Up No Limit Hold'em

A production-grade WebSocket-based poker server in Rust supporting heads-up (2-player) No Limit Texas Hold'em cash games.

## 🎯 Current Status

**Phase 1 & 2 COMPLETE** ✅  
**Phase 3-5: In Progress** 🚧

### ✅ Completed Features

#### Phase 1: Project Foundation
- **Dependencies**: All Rust crates configured (tokio, serde, sqlx, argon2, etc.)
- **Project Structure**: Organized modules for models, database, game logic, websockets, table management
- **Error Handling**: Comprehensive error types using `thiserror`
- **Configuration**: Server settings with sensible defaults
- **Data Models**: 
  - Card, Deck, Rank, Suit with serialization
  - Player with authentication and chip management
  - GameState with full poker game representation
  - Table management
  - WebSocket message protocol (JSON)
- **Database Layer**: 
  - SQLite schema for players, tables, game sessions
  - Argon2 password hashing
  - Full CRUD operations for players
  - Type-safe queries with `sqlx`

#### Phase 2: Game Logic Engine  
- **Hand Evaluator** (all 10 poker hand rankings):
  - Royal Flush, Straight Flush, Four of a Kind
  - Full House, Flush, Straight
  - Three of a Kind, Two Pair, Pair, High Card
  - Tie-breaking logic with kickers
  - 7-card best-5 selection (21 combinations)
  - Special cases: wheel straight (A-2-3-4-5)
  
- **Dealer Logic**:
  - Cryptographically secure shuffle
  - Blind posting (heads-up rules)
  - Hole card dealing
  - Flop, turn, river with burn cards
  - Pot management

- **Betting Rules**:
  - Action validation (fold, check, call, raise, all-in)
  - Minimum/maximum raise enforcement
  - All-in detection and handling
  - Round completion detection
  - Valid action generation

### 🚧 Remaining Work

#### Phase 3: WebSocket Server (TODO)
- WebSocket listener on port 8080
- Session management with authentication
- Message routing and broadcasting
- Heartbeat/ping-pong mechanism

#### Phase 4: Table Manager (TODO)
- Dynamic table creation/destruction
- 5 concurrent table limit enforcement
- Player seating and game startup
- Disconnection/reconnection handling (30s grace period)
- Buy-in validation and chip management
- Play money faucet (top-up to 100)

#### Phase 5: Testing & Hardening (TODO)
- Integration tests (end-to-end game flows)
- Load/stress tests (5 concurrent tables, 100 hands each)
- Security hardening (input validation, fuzzing, rate limiting)
- CI/CD pipeline

## 📊 Test Results

```
Running 43 tests across all modules:
✅ All tests passing
```

**Test Coverage:**
- Card/Deck operations: 3 tests
- Player chip management: 3 tests  
- Game state management: 3 tests
- Database operations: 4 tests
- Hand evaluator (all 10 rankings): 12 tests
- Dealer logic: 5 tests
- Betting rules: 13 tests

## 🏗️ Architecture

```
poker-server/
├── src/
│   ├── main.rs                 # Entry point (minimal)
│   ├── lib.rs                  # Library exports
│   ├── config.rs               # Server configuration
│   ├── error.rs                # Custom error types
│   ├── models/                 # Data models
│   │   ├── card.rs            # Card, Deck, Rank, Suit
│   │   ├── player.rs          # Player account & session
│   │   ├── game.rs            # GameState, PlayerGameState
│   │   ├── table.rs           # Table model
│   │   └── messages.rs        # WebSocket message types
│   ├── db/
│   │   └── sqlite.rs          # Database layer with Argon2
│   ├── game_logic/
│   │   ├── hand_evaluator.rs  # Poker hand ranking (43 LOC tests!)
│   │   ├── dealer.rs          # Card dealing & blinds
│   │   └── betting.rs         # Action validation & rules
│   ├── websocket/             # (Stubs - Phase 3)
│   └── table_manager/         # (Stubs - Phase 4)
├── tests/
│   ├── integration/           # (TODO - Phase 5)
│   └── load/                  # (TODO - Phase 5)
└── Cargo.toml                 # Dependencies configured
```

## 🚀 Running

```bash
# Run tests
cargo test

# Build
cargo build

# Run server (database initialization only - WebSocket server TODO)
cargo run
```

##  💾 Database Schema

```sql
CREATE TABLE players (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,  -- Argon2
    chips INTEGER DEFAULT 100,
    hands_played INTEGER DEFAULT 0,
    hands_won INTEGER DEFAULT 0,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

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

## 🎮 Game Rules (MVP Scope)

- **Game Type**: Heads-Up (2-player) No Limit Texas Hold'em
- **Blinds**: Fixed at 0.5/1.0
- **Buy-in**: 20-100 big blinds (20-100 chips)
- **Currency**: Play money with faucet (top-up to 100 when below 100)
- **Tables**: Up to 5 concurrent tables
- **Disconnection**: 30-second grace period for reconnection

## 📝 Next Steps

1. **Implement WebSocket Server** (Phase 3)
   - Set up `tokio-tungstenite` listener
   - Create session management with player authentication
   - Implement message handlers for all client actions
   
2. **Build Table Manager** (Phase 4)
   - Multi-table orchestration with Arc<RwLock<>>
   - Disconnection handling with grace period
   - Buy-in validation and faucet logic

3. **Comprehensive Testing** (Phase 5)
   - End-to-end integration tests
   - Load testing with 5 concurrent games
   - Security fuzzing and hardening
   
4. **Deployment**
   - Systemd service configuration
   - nginx reverse proxy
   - Production database setup

## 🛠️ Technology Stack

- **Runtime**: Tokio (async I/O)
- **WebSocket**: tokio-tungstenite
- **Serialization**: serde + serde_json  
- **Database**: SQLite with sqlx (type-safe queries)
- **Authentication**: Argon2 password hashing
- **Error Handling**: thiserror + anyhow
- **Testing**: Built-in Rust testing + proptest (property-based)

---

**Built with Rust 🦀 | Following the tech-spec from `docs/sprint-artifacts/tech-spec-poker-server.md`**
