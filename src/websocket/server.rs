//! WebSocket server implementation.
//!
//! This module will provide the WebSocket communication layer for the poker server.
//! Key responsibilities:
//! - WebSocket listener on configured port (default 8080)
//! - Session management with player authentication
//! - Message routing and broadcasting to connected clients
//! - Heartbeat/ping-pong mechanism for connection health
//!
//! Implementation requirements:
//! - Use tokio-tungstenite for WebSocket support
//! - Handle ClientMessage deserialization and ServerMessage serialization
//! - Implement graceful connection handling with proper cleanup
//! - Rate limiting and connection validation
//!
//! Reference: docs/sprint-artifacts/tech-spec-poker-server.md

// WebSocket server - to be implemented in Phase 3
//
// This module will provide the WebSocket communication layer for the poker server.
// Key responsibilities:
// - WebSocket listener on configured port (default 8080)
// - Session management with player authentication
// - Message routing and broadcasting to connected clients
// - Heartbeat/ping-pong mechanism for connection health
//
// Implementation requirements:
// - Use tokio-tungstenite for WebSocket support
// - Handle ClientMessage deserialization and ServerMessage serialization
// - Implement graceful connection handling with proper cleanup
// - Rate limiting and connection validation
//
// Reference: docs/sprint-artifacts/tech-spec-poker-server.md
