// Table manager - to be implemented in Phase 4
//
// This module will provide table management and orchestration for the poker server.
// Key responsibilities:
// - Dynamic table creation and destruction (up to max_tables limit)
// - Player seating and game startup coordination
// - Disconnection/reconnection handling (with grace period)
// - Buy-in validation and chip management
// - Play money faucet (top-up to threshold)
//
// Implementation requirements:
// - Use Arc<RwLock<>> for thread-safe shared state
// - Track player sessions and table assignments
// - Manage game state transitions
// - Handle all-in side pot calculation (see models/game.rs TODO)
//
// Reference: docs/sprint-artifacts/tech-spec-poker-server.md
