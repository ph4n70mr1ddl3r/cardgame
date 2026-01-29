use poker_server::{db::Database, init_logging, Config};

#[tokio::main]
async fn main() -> poker_server::error::Result<()> {
    init_logging();
    tracing::info!("Poker Server Starting...");

    let config = Config::from_env()?;
    tracing::info!(
        "Server will run on {}:{}",
        config.server_host,
        config.server_port
    );
    tracing::info!("Database: {}", config.database_url);

    let db = Database::new(&config.database_url, config.starting_chips, config.db_max_connections).await?;
    db.initialize_schema().await?;
    tracing::info!("Database initialized");
    tracing::info!("Game logic engine ready");
    tracing::info!("Hand evaluator: All 10 poker hand rankings");
    tracing::info!("Dealer: Deck shuffling, card dealing, blind posting");
    tracing::info!("Betting: Action validation, raise rules, all-in handling");

    Ok(())
}
