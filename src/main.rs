use poker_server::error::{PokerError, Result};
use poker_server::{db::Database, init_logging, Config};

#[tokio::main]
async fn main() -> Result<()> {
    init_logging();
    tracing::info!("Poker Server Starting...");

    let config = Config::default();
    config
        .validate()
        .map_err(|e| PokerError::Game(format!("Config validation failed: {}", e)))?;
    tracing::info!(
        "Server will run on {}:{}",
        config.server_host,
        config.server_port
    );
    tracing::info!("Database: {}", config.database_url);

    let db = Database::new(&config.database_url, config.starting_chips)
        .await
        .map_err(|e| PokerError::Game(format!("Failed to connect to database: {}", e)))?;
    db.initialize_schema().await?;
    tracing::info!("Database initialized");

    tracing::info!("WebSocket server not yet implemented");
    tracing::info!("Game logic engine ready:");
    tracing::info!("   - Hand evaluator: All 10 poker hand rankings");
    tracing::info!("   - Dealer: Deck shuffling, card dealing, blind posting");
    tracing::info!("   - Betting: Action validation, raise rules, all-in handling");

    tracing::info!("To complete implementation:");
    tracing::info!("   - Phase 3: WebSocket server with session management");
    tracing::info!("   - Phase 4: Table manager with concurrency support");
    tracing::info!("   - Phase 5: Comprehensive testing and hardening");

    Ok(())
}
