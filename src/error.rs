use thiserror::Error;

#[derive(Error, Debug)]
pub enum PokerError {
    #[error("Network error: {0}")]
    Network(String),
    
    #[error("Game logic error: {0}")]
    Game(String),
    
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("Authentication error: {0}")]
    Auth(String),
    
    #[error("Invalid action: {0}")]
    InvalidAction(String),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("Password hashing error: {0}")]
    PasswordHash(String),
    
    #[error("WebSocket error: {0}")]
    WebSocket(String),
}

pub type Result<T> = std::result::Result<T, PokerError>;
