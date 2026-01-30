use thiserror::Error;

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum PokerError {
    #[error("Network error: {message}")]
    Network { message: String },

    #[error("Game logic error: {message}")]
    Game { message: String },

    #[error("Game validation failed for '{field}': {value}")]
    GameValidation { field: String, value: String },

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Authentication error: {reason}")]
    Auth { reason: String },

    #[error("Invalid action: {reason}")]
    InvalidAction { reason: String },

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Password hashing error: {reason}")]
    PasswordHash { reason: String },

    #[error("WebSocket error: {reason}")]
    WebSocket { reason: String },

    #[error("Invalid player index {index} (max {max})")]
    InvalidPlayerIndex { index: usize, max: usize },
}

impl PokerError {
    pub fn network<S: Into<String>>(message: S) -> Self {
        Self::Network {
            message: message.into(),
        }
    }

    pub fn game<S: Into<String>>(message: S) -> Self {
        Self::Game {
            message: message.into(),
        }
    }

    pub fn game_validation<S: Into<String>>(field: S, value: S) -> Self {
        Self::GameValidation {
            field: field.into(),
            value: value.into(),
        }
    }

    pub fn auth<S: Into<String>>(reason: S) -> Self {
        Self::Auth {
            reason: reason.into(),
        }
    }

    pub fn invalid_action<S: Into<String>>(reason: S) -> Self {
        Self::InvalidAction {
            reason: reason.into(),
        }
    }

    pub fn password_hash<S: Into<String>>(reason: S) -> Self {
        Self::PasswordHash {
            reason: reason.into(),
        }
    }

    pub fn websocket<S: Into<String>>(reason: S) -> Self {
        Self::WebSocket {
            reason: reason.into(),
        }
    }

    pub fn invalid_player_index(index: usize, max: usize) -> Self {
        Self::InvalidPlayerIndex { index, max }
    }
}

pub type Result<T> = std::result::Result<T, PokerError>;
