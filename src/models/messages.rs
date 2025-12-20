use super::card::Card;
use super::game::{GameStage, PlayerAction, PlayerGameState};
use serde::{Deserialize, Serialize};

// Client -> Server Messages
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ClientMessage {
    Signup { username: String, password: String },
    Login { username: String, password: String },
    CreateTable { name: String },
    JoinTable { table_id: i64, buyin: i64 },
    LeaveTable,
    GameAction { action: PlayerAction },
    TopUp,
    Ping,
}

// Server -> Client Messages
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ServerMessage {
    SignupResponse { success: bool, message: String, player_id: Option<i64> },
    LoginResponse { success: bool, message: String, player_id: Option<i64>, chips: Option<i64> },
    TableCreated { table_id: i64, name: String },
    TableJoined { table_id: i64, seat_index: usize },
    TableLeft { table_id: i64 },
    GameStarted { hand_number: u64 },
    GameStateUpdate { 
        stage: GameStage,
        players: Vec<PlayerGameState>,
        community_cards: Vec<Card>,
        pot: i64,
        current_bet: i64,
        current_player_index: Option<usize>,
    },
    HoleCards { cards: Vec<Card> },
    ActionRequired { valid_actions: Vec<String>, min_raise: Option<i64>, max_raise: Option<i64> },
    PlayerAction { player_id: i64, username: String, action: PlayerAction },
    HandResult { 
        winner_id: i64,
        winner_username: String,
        winning_hand: String,
        pot_won: i64,
    },
    TopUpResponse { success: bool, new_balance: Option<i64>, message: String },
    OpponentDisconnected { grace_period_secs: u64 },
    OpponentReconnected,
    Error { message: String },
    Pong,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_message_deserialization() {
        let json = r#"{"type": "login", "username": "test", "password": "pass123"}"#;
        let msg: ClientMessage = serde_json::from_str(json).unwrap();
        
        match msg {
            ClientMessage::Login { username, password } => {
                assert_eq!(username, "test");
                assert_eq!(password, "pass123");
            }
            _ => panic!("Wrong message type"),
        }
    }

    #[test]
    fn test_server_message_serialization() {
        let msg = ServerMessage::LoginResponse {
            success: true,
            message: "Welcome!".to_string(),
            player_id: Some(1),
            chips: Some(100),
        };
        
        let json = serde_json::to_string(&msg).unwrap();
        assert!(json.contains("login_response"));
        assert!(json.contains("Welcome!"));
    }
}
