use poker_server::error::{PokerError, Result};
use poker_server::{db::Database, Config};

#[tokio::main]
async fn main() -> Result<()> {
    println!("🃏 Poker Server Starting...");

    let config = Config::default();
    config
        .validate()
        .map_err(|e| PokerError::Game(format!("Config validation failed: {}", e)))?;
    println!(
        "📡 Server will run on {}:{}",
        config.server_host, config.server_port
    );
    println!("💾 Database: {}", config.database_url);

    // Initialize database
    let db = Database::new(&config.database_url).await?;
    db.initialize_schema().await?;
    println!("✅ Database initialized");

    println!("⚠️  WebSocket server not yet implemented");
    println!("✅ Game logic engine ready:");
    println!("   - Hand evaluator: All 10 poker hand rankings");
    println!("   - Dealer: Deck shuffling, card dealing, blind posting");
    println!("   - Betting: Action validation, raise rules, all-in handling");

    println!("\n🚧 To complete implementation:");
    println!("   - Phase 3: WebSocket server with session management");
    println!("   - Phase 4: Table manager with concurrency support");
    println!("   - Phase 5: Comprehensive testing and hardening");

    Ok(())
}
