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
}

impl Database {
    pub async fn new(database_url: &str) -> Result<Self> {
        let pool = SqlitePool::connect(database_url).await?;
        Ok(Self { pool })
    }

    pub async fn initialize_schema(&self) -> Result<()> {
        // Create players table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS players (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                username TEXT UNIQUE NOT NULL,
                password_hash TEXT NOT NULL,
                chips INTEGER DEFAULT 100,
                hands_played INTEGER DEFAULT 0,
                hands_won INTEGER DEFAULT 0,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        sqlx::query("CREATE INDEX IF NOT EXISTS idx_players_username ON players(username)")
            .execute(&self.pool)
            .await?;

        // Create tables table
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

        // Create game_sessions table
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

        Ok(())
    }

    // Player CRUD Operations
    pub async fn create_player(&self, username: &str, password: &str) -> Result<i64> {
        if username.len() < 3 || username.len() > 20 {
            return Err(crate::error::PokerError::Game(
                "Username must be between 3 and 20 characters".to_string(),
            ));
        }
        if password.len() < 8 || password.len() > 128 {
            return Err(crate::error::PokerError::Game(
                "Password must be between 8 and 128 characters".to_string(),
            ));
        }
        let password_hash = self.hash_password(password)?;

        let result = sqlx::query("INSERT INTO players (username, password_hash) VALUES (?, ?)")
            .bind(username)
            .bind(&password_hash)
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

    pub async fn update_player_chips(&self, player_id: i64, new_chips: i64) -> Result<()> {
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

        Ok(Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok()
            .then_some(player))
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
        let db = Database::new("sqlite::memory:").await.unwrap();
        db.initialize_schema().await.unwrap();
        db
    }

    #[tokio::test]
    async fn test_create_and_get_player() {
        let db = setup_test_db().await;

        let player_id = db.create_player("testuser", "password123").await.unwrap();
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

        db.create_player("user1", "correctpass").await.unwrap();

        let valid = db.verify_password("user1", "correctpass").await.unwrap();
        assert!(valid.is_some());

        let invalid = db.verify_password("user1", "wrongpass").await.unwrap();
        assert!(invalid.is_none());
    }

    #[tokio::test]
    async fn test_update_player_chips() {
        let db = setup_test_db().await;

        let player_id = db.create_player("user2", "password123").await.unwrap();
        db.update_player_chips(player_id, 250).await.unwrap();

        let player = db.get_player_by_id(player_id).await.unwrap().unwrap();
        assert_eq!(player.chips, 250);
    }

    #[tokio::test]
    async fn test_update_player_stats() {
        let db = setup_test_db().await;

        let player_id = db.create_player("user3", "password456").await.unwrap();
        db.update_player_stats(player_id, 5, 2).await.unwrap();

        let player = db.get_player_by_id(player_id).await.unwrap().unwrap();
        assert_eq!(player.hands_played, 5);
        assert_eq!(player.hands_won, 2);
    }
}
