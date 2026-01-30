#![deny(clippy::all)]

//! Poker Server - A heads-up Texas Hold'em poker server written in Rust.
//!
//! This library provides the core functionality for a poker server including:
//! - Game logic (hand evaluation, dealing, betting)
//! - Database persistence (SQLite)
//! - Configuration management
//! - WebSocket message types (implementation pending)
//!
//! # Architecture
//!
//! The server is organized into several modules:
//! - `config`: Configuration from environment variables
//! - `db`: Database abstraction layer with SQLite backend
//! - `error`: Comprehensive error types
//! - `game_logic`: Poker game rules and mechanics
//! - `models`: Core data structures
//! - `password_policy`: Password validation
//! - `table_manager`: Table orchestration (pending)
//! - `websocket`: WebSocket communication (pending)

pub mod config;
pub mod db;
pub mod error;
pub mod game_logic;
pub mod logger;
pub mod models;
pub mod password_policy;
pub mod table_manager;
pub mod websocket;

pub use config::Config;
pub use error::{PokerError, Result};
pub use logger::init_logging;
pub use models::game::ValidAction;
