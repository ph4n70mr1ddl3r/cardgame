//! Database module for persistent storage.
//!
//! This module provides an abstraction layer over SQLite for storing
//! player data, tables, and game sessions.

pub mod sqlite;

pub use sqlite::Database;
