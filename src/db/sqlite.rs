use crate::error::Result;
use crate::models::Player;
use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use rand::rngs::OsRng;
use sqlx::{sqlite::SqlitePool, Row};

pub struct Database {
    pool: SqlitePool,
    starting_chips: i64,
}

impl Database {
    pub async fn new(database_url: &str, starting_chips: i64) -> Result<Self> {
        let pool = SqlitePool::connect(database_url).await?;
        Ok(Self {
            pool,
            starting_chips,
        })
    }

    pub async fn initialize_schema(&self) -> Result<()> {
        // Create players table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS players (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                username TEXT UNIQUE NOT NULL,
                password_hash TEXT NOT NULL,
                chips INTEGER DEFAULT 100 CHECK(chips >= 0),
                hands_played INTEGER DEFAULT 0 CHECK(hands_played >= 0),
                hands_won INTEGER DEFAULT 0 CHECK(hands_won >= 0),
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        sqlx::query("CREATE INDEX IF NOT EXISTS idx_players_username ON players(username)")
            .execute(&self.pool)
            .await?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS tables (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                small_blind INTEGER NOT NULL,
                big_blind INTEGER NOT NULL,
                max_players INTEGER DEFAULT 2,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        sqlx::query("CREATE INDEX IF NOT EXISTS idx_tables_name ON tables(name)")
            .execute(&self.pool)
            .await?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS game_sessions (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                table_id INTEGER NOT NULL,
                started_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                ended_at DATETIME,
                FOREIGN KEY (table_id) REFERENCES tables(id)
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        sqlx::query(
            "CREATE INDEX IF NOT EXISTS idx_game_sessions_table_id ON game_sessions(table_id)",
        )
        .execute(&self.pool)
        .await?;

        sqlx::query(
            "CREATE INDEX IF NOT EXISTS idx_game_sessions_started_at ON game_sessions(started_at)",
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    // Player CRUD Operations
    pub async fn create_player(&self, username: &str, password: &str) -> Result<i64> {
        if username.len() < crate::models::game::MIN_USERNAME_LEN
            || username.len() > crate::models::game::MAX_USERNAME_LEN
        {
            return Err(crate::error::PokerError::Game(format!(
                "Username must be between {} and {} characters",
                crate::models::game::MIN_USERNAME_LEN,
                crate::models::game::MAX_USERNAME_LEN
            )));
        }
        if !username.chars().all(|c| c.is_alphanumeric() || c == '_') {
            return Err(crate::error::PokerError::Game(
                "Username can only contain letters, numbers, and underscores".to_string(),
            ));
        }
        if !username
            .chars()
            .next()
            .map(|c| c.is_alphabetic())
            .unwrap_or(false)
        {
            return Err(crate::error::PokerError::Game(
                "Username must start with a letter".to_string(),
            ));
        }
        if password.len() < crate::models::game::MIN_PASSWORD_LEN
            || password.len() > crate::models::game::MAX_PASSWORD_LEN
        {
            return Err(crate::error::PokerError::Game(format!(
                "Password must be between {} and {} characters",
                crate::models::game::MIN_PASSWORD_LEN,
                crate::models::game::MAX_PASSWORD_LEN
            )));
        }
        let has_upper = password.chars().any(|c| c.is_uppercase());
        let has_lower = password.chars().any(|c| c.is_lowercase());
        let has_digit = password.chars().any(|c| c.is_ascii_digit());
        if !has_upper || !has_lower || !has_digit {
            return Err(crate::error::PokerError::Game(
                "Password must contain at least one uppercase letter, one lowercase letter, and one digit".to_string(),
            ));
        }
        let password_hash = self.hash_password(password)?;

        let result =
            sqlx::query("INSERT INTO players (username, password_hash, chips) VALUES (?, ?, ?)")
                .bind(username)
                .bind(&password_hash)
                .bind(self.starting_chips)
                .execute(&self.pool)
                .await?;

        Ok(result.last_insert_rowid())
    }

    pub async fn get_player_by_username(&self, username: &str) -> Result<Option<Player>> {
        let row = sqlx::query(
            "SELECT id, username, password_hash, chips, hands_played, hands_won FROM players WHERE username = ?"
        )
        .bind(username)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| Player {
            id: r.get("id"),
            username: r.get("username"),
            password_hash: r.get("password_hash"),
            chips: r.get("chips"),
            hands_played: r.get("hands_played"),
            hands_won: r.get("hands_won"),
        }))
    }

    pub async fn get_player_by_id(&self, player_id: i64) -> Result<Option<Player>> {
        let row = sqlx::query(
            "SELECT id, username, password_hash, chips, hands_played, hands_won FROM players WHERE id = ?"
        )
        .bind(player_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| Player {
            id: r.get("id"),
            username: r.get("username"),
            password_hash: r.get("password_hash"),
            chips: r.get("chips"),
            hands_played: r.get("hands_played"),
            hands_won: r.get("hands_won"),
        }))
    }

    pub async fn update_player_chips(&self, player_id: i64, delta: i64) -> Result<()> {
        let Some(new_chips) =
            sqlx::query_scalar::<_, i64>("SELECT chips FROM players WHERE id = ?")
                .bind(player_id)
                .fetch_optional(&self.pool)
                .await?
                .and_then(|chips| chips.checked_add(delta))
        else {
            return Err(crate::error::PokerError::Game(
                "Chip overflow or player not found".to_string(),
            ));
        };

        if new_chips < 0 {
            return Err(crate::error::PokerError::Game(
                "Insufficient chips".to_string(),
            ));
        }

        sqlx::query("UPDATE players SET chips = ? WHERE id = ?")
            .bind(new_chips)
            .bind(player_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn update_player_stats(
        &self,
        player_id: i64,
        hands_played_delta: i64,
        hands_won_delta: i64,
    ) -> Result<()> {
        if hands_played_delta < 0 || hands_won_delta < 0 {
            return Err(crate::error::PokerError::Game(
                "Stat deltas cannot be negative".to_string(),
            ));
        }
        sqlx::query(
            "UPDATE players SET hands_played = hands_played + ?, hands_won = hands_won + ? WHERE id = ?"
        )
        .bind(hands_played_delta)
        .bind(hands_won_delta)
        .bind(player_id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn verify_password(&self, username: &str, password: &str) -> Result<Option<Player>> {
        let player = match self.get_player_by_username(username).await? {
            Some(p) => p,
            None => return Ok(None),
        };

        let parsed_hash = PasswordHash::new(&player.password_hash).map_err(|e| {
            crate::error::PokerError::Auth(format!("Invalid password hash format: {}", e))
        })?;

        let is_valid = Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok();

        if is_valid {
            Ok(Some(player))
        } else {
            Ok(None)
        }
    }

    // Helper functions
    fn hash_password(&self, password: &str) -> Result<String> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| crate::error::PokerError::PasswordHash(e.to_string()))?
            .to_string();
        Ok(hash)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    async fn setup_test_db() -> Database {
        let db = Database::new("sqlite::memory:", 100).await.unwrap();
        db.initialize_schema().await.unwrap();
        db
    }

    #[tokio::test]
    async fn test_create_and_get_player() {
        let db = setup_test_db().await;

        let player_id = db.create_player("testuser", "Password123").await.unwrap();
        assert!(player_id > 0);

        let player = db.get_player_by_username("testuser").await.unwrap();
        assert!(player.is_some());

        let player = player.unwrap();
        assert_eq!(player.username, "testuser");
        assert_eq!(player.chips, 100);
    }

    #[tokio::test]
    async fn test_password_verification() {
        let db = setup_test_db().await;

        db.create_player("user1", "CorrectPass123").await.unwrap();

        let valid = db.verify_password("user1", "CorrectPass123").await.unwrap();
        assert!(valid.is_some());

        let invalid = db.verify_password("user1", "wrongpass").await.unwrap();
        assert!(invalid.is_none());
    }

    #[tokio::test]
    async fn test_update_player_chips() {
        let db = setup_test_db().await;

        let player_id = db.create_player("user2", "Password123").await.unwrap();
        db.update_player_chips(player_id, 150).await.unwrap();

        let player = db.get_player_by_id(player_id).await.unwrap().unwrap();
        assert_eq!(player.chips, 250);
    }

    #[tokio::test]
    async fn test_update_player_stats() {
        let db = setup_test_db().await;

        let player_id = db.create_player("user3", "Password456").await.unwrap();
        db.update_player_stats(player_id, 5, 2).await.unwrap();

        let player = db.get_player_by_id(player_id).await.unwrap().unwrap();
        assert_eq!(player.hands_played, 5);
        assert_eq!(player.hands_won, 2);
    }

    #[tokio::test]
    async fn test_invalid_username() {
        let db = setup_test_db().await;

        let result = db.create_player("123user", "Password123").await;
        assert!(result.is_err());

        let result = db.create_player("", "Password123").await;
        assert!(result.is_err());
    }
}
