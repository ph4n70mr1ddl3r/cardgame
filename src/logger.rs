//! Logging module for the poker server.
//!
//! This module provides structured logging using the tracing crate.
//! Log levels can be configured via the RUST_LOG environment variable.

use tracing::{info, Level};
use tracing_subscriber::{fmt, EnvFilter};

/// Initializes logging for the poker server.
///
/// Sets up structured logging with tracing, configurable via environment variables:
/// - `RUST_LOG`: Controls log level (e.g., `info`, `debug`, `warn`)
/// - Defaults to INFO level
pub fn init_logging() {
    let env_filter = EnvFilter::builder()
        .with_default_directive(Level::INFO.into())
        .from_env_lossy();

    fmt()
        .with_env_filter(env_filter)
        .with_target(false)
        .with_level(true)
        .init();

    info!("Logging initialized");
}
