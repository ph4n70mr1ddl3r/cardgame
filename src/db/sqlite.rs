use crate::error::Result;
use crate::models::Player;
use crate::password_policy;
use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use rand::rngs::OsRng;
use sqlx::{sqlite::SqlitePool, Row};

/// Database layer for persistent storage
pub struct Database {
    pool: SqlitePool,
    starting_chips: i64,
    max_connections: u32,
}

impl Database {
    /// Creates a new database connection pool.
    ///
    /// # Arguments
    ///
    /// * `database_url` - SQLite database connection string
    /// * `starting_chips` - Initial chips for new players
    /// * `max_connections` - Maximum pool size
    pub async fn new(
        database_url: &str,
        starting_chips: i64,
        max_connections: u32,
    ) -> Result<Self> {
        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .max_connections(max_connections)
            .acquire_timeout(std::time::Duration::from_secs(30))
            .connect(database_url)
            .await
            .map_err(|e| {
                crate::error::PokerError::game(format!("Database connection failed: {}", e))
            })?;
        Ok(Self {
            pool,
            starting_chips,
            max_connections,
        })
    }

    /// Returns the maximum number of connections configured for this pool.
    pub fn max_connections(&self) -> u32 {
        self.max_connections
    }

    /// Checks database connectivity and returns true if healthy.
    ///
    /// Executes a simple query to verify the database connection is working.
    #[must_use = "health check results should always be checked"]
    pub async fn health_check(&self) -> bool {
        sqlx::query("SELECT 1")
            .fetch_one(&self.pool)
            .await
            .is_ok()
    }

    /// Creates database tables and indexes if they don't exist.
    ///
    /// Initializes the following tables:
    /// - `players`: Stores player accounts and statistics
    /// - `tables`: Stores poker table configurations
    /// - `game_sessions`: Tracks game sessions for analytics
    pub async fn initialize_schema(&self) -> Result<()> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS players (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                username TEXT UNIQUE NOT NULL,
                password_hash TEXT NOT NULL,
                chips INTEGER NOT NULL CHECK(chips >= 0),
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

        sqlx::query("CREATE INDEX IF NOT EXISTS idx_players_chips ON players(chips)")
            .execute(&self.pool)
            .await?;

        sqlx::query("CREATE INDEX IF NOT EXISTS idx_players_created_at ON players(created_at)")
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

    /// Creates a new player account with username and password.
    ///
    /// Validates username format and password strength, hashes the password
    /// using Argon2, and stores the player with starting chips.
    pub async fn create_player(&self, username: &str, password: &str) -> Result<i64> {
        Self::validate_username(username)?;
        password_policy::validate_password(password).map_err(|e| {
            crate::error::PokerError::game(format!("Password validation failed: {e}"))
        })?;
        let password_hash = Self::hash_password(password)?;

        let result = match sqlx::query(
            "INSERT INTO players (username, password_hash, chips) VALUES (?, ?, ?)"
        )
        .bind(username)
        .bind(&password_hash)
        .bind(self.starting_chips)
        .execute(&self.pool)
        .await {
            Ok(r) => r,
            Err(sqlx::Error::Database(err)) if err.message().contains("UNIQUE constraint failed") => {
                return Err(crate::error::PokerError::game(
                    format!("Username '{}' is already taken", username)
                ));
            }
            Err(e) => return Err(crate::error::PokerError::game(format!("Failed to create player: {}", e))),
        };

        Ok(result.last_insert_rowid())
    }

    /// Retrieves a player by their username.
    ///
    /// Returns None if the player doesn't exist.
    #[must_use = "player lookup result should always be checked"]
    pub async fn get_player_by_username(&self, username: &str) -> Result<Option<Player>> {
        let row = sqlx::query(
            "SELECT id, username, password_hash, chips, hands_played, hands_won FROM players WHERE username = ?"
        )
        .bind(username)
        .fetch_optional(&self.pool)
        .await?;

        if let Some(r) = &row {
            let chips: i64 = r.get("chips");
            if chips < 0 {
                return Err(crate::error::PokerError::game(format!(
                    "Data corruption: player '{}' has negative chips: {}",
                    username, chips
                )));
            }
        }

        Ok(row.map(|r| Player {
            id: r.get("id"),
            username: r.get("username"),
            password_hash: r.get("password_hash"),
            chips: r.get("chips"),
            hands_played: r.get("hands_played"),
            hands_won: r.get("hands_won"),
        }))
    }

    /// Retrieves a player by their database ID.
    ///
    /// Returns None if the player doesn't exist.
    #[must_use = "player lookup result should always be checked"]
    pub async fn get_player_by_id(&self, player_id: i64) -> Result<Option<Player>> {
        let row = sqlx::query(
            "SELECT id, username, password_hash, chips, hands_played, hands_won FROM players WHERE id = ?"
        )
        .bind(player_id)
        .fetch_optional(&self.pool)
        .await?;

        if let Some(r) = &row {
            let chips: i64 = r.get("chips");
            if chips < 0 {
                return Err(crate::error::PokerError::game(format!(
                    "Data corruption: player ID {} has negative chips: {}",
                    player_id, chips
                )));
            }
        }

        Ok(row.map(|r| Player {
            id: r.get("id"),
            username: r.get("username"),
            password_hash: r.get("password_hash"),
            chips: r.get("chips"),
            hands_played: r.get("hands_played"),
            hands_won: r.get("hands_won"),
        }))
    }

    /// Updates a player's chip balance by adding delta (can be negative).
    ///
    /// Uses database-level CHECK constraint to prevent negative chip balance.
    ///
    /// # Arguments
    ///
    /// * `player_id` - ID of player to update
    /// * `delta` - Amount to add (positive) or subtract (negative)
    ///
    /// # Returns
    ///
    /// * `Result<()>` - Ok if update successful
    ///
    /// # Errors
    ///
    /// Returns error if:
    /// - Player not found
    /// - Resulting balance would be negative
    /// - Database query fails
    pub async fn update_player_chips(&self, player_id: i64, delta: i64) -> Result<()> {
        let result =
            sqlx::query("UPDATE players SET chips = chips + ? WHERE id = ? AND chips + ? >= 0")
                .bind(delta)
                .bind(player_id)
                .bind(delta)
                .execute(&self.pool)
                .await?;

        if result.rows_affected() == 0 {
            return Err(crate::error::PokerError::game(
                "Insufficient chips or player not found",
            ));
        }

        Ok(())
    }

    /// Updates a player's game statistics.
    ///
    /// Increments hands_played and hands_won by the specified deltas.
    pub async fn update_player_stats(
        &self,
        player_id: i64,
        hands_played_delta: i64,
        hands_won_delta: i64,
    ) -> Result<()> {
        if hands_played_delta < 0 || hands_won_delta < 0 {
            return Err(crate::error::PokerError::game(
                "Stat deltas cannot be negative",
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

    /// Updates a player's chips and stats atomically within a transaction.
    ///
    /// This is useful for game completion where both chips and statistics need to be updated together.
    ///
    /// # Arguments
    ///
    /// * `player_id` - ID of player to update
    /// * `chip_delta` - Amount to add (positive) or subtract (negative)
    /// * `hands_played_delta` - Number of hands played to add
    /// * `hands_won_delta` - Number of hands won to add
    pub async fn update_chips_and_stats(
        &self,
        player_id: i64,
        chip_delta: i64,
        hands_played_delta: i64,
        hands_won_delta: i64,
    ) -> Result<()> {
        self.transaction(|tx| Box::pin(async move {
            sqlx::query("UPDATE players SET chips = chips + ? WHERE id = ? AND chips + ? >= 0")
                .bind(chip_delta)
                .bind(player_id)
                .bind(chip_delta)
                .execute(&mut **tx)
                .await
                .map_err(|e| {
                    crate::error::PokerError::game(format!("Failed to update chips: {}", e))
                })?;

            if hands_played_delta < 0 || hands_won_delta < 0 {
                return Err(crate::error::PokerError::game(
                    "Stat deltas cannot be negative",
                ));
            }

            sqlx::query(
                "UPDATE players SET hands_played = hands_played + ?, hands_won = hands_won + ? WHERE id = ?"
            )
            .bind(hands_played_delta)
            .bind(hands_won_delta)
            .bind(player_id)
            .execute(&mut **tx)
            .await?;

            Ok(())
        })).await
    }

    /// Verifies a player's password by comparing against stored hash.
    ///
    /// Uses Argon2 to verify the password hash. Returns the player if valid,
    /// None if password is incorrect or player doesn't exist.
    ///
    /// # Arguments
    ///
    /// * `username` - Username to look up
    /// * `password` - Plain text password to verify
    ///
    /// # Returns
    ///
    /// * `Result<Option<Player>>` - Some(Player) if password valid, None if invalid or player not found
    ///
    /// # Errors
    ///
    /// Returns error if database query fails or password hash is corrupted
    #[must_use = "authentication results should always be checked"]
    pub async fn verify_password(&self, username: &str, password: &str) -> Result<Option<Player>> {
        let Some(player) = self.get_player_by_username(username).await? else {
            return Ok(None);
        };

        let parsed_hash = PasswordHash::new(&player.password_hash).map_err(|e| {
            crate::error::PokerError::auth(format!("Invalid password hash format: {e}"))
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

    /// Validates that a username meets format requirements.
    ///
    /// Rules:
    /// - 3-20 characters
    /// - Must start with a letter
    /// - Only alphanumeric characters and underscores allowed
    /// - No control characters or bidirectional override characters
    fn validate_username(username: &str) -> Result<()> {
        if username.len() < crate::models::game::MIN_USERNAME_LEN
            || username.len() > crate::models::game::MAX_USERNAME_LEN
        {
            return Err(crate::error::PokerError::game(format!(
                "Username must be between {} and {} characters",
                crate::models::game::MIN_USERNAME_LEN,
                crate::models::game::MAX_USERNAME_LEN
            )));
        }
        if !username.chars().next().is_some_and(char::is_alphabetic) {
            return Err(crate::error::PokerError::game(
                "Username must start with a letter",
            ));
        }
        if !username.chars().all(|c| c.is_alphanumeric() || c == '_') {
            return Err(crate::error::PokerError::game(
                "Username can only contain letters, numbers, and underscores",
            ));
        }
        if username.chars().any(|c| {
            c.is_control() || matches!(c, '\u{200B}'..='\u{200F}' | '\u{202A}'..='\u{202E}')
        }) {
            return Err(crate::error::PokerError::game(
                "Username contains invalid characters",
            ));
        }
        Ok(())
    }

    /// Hashes a password using Argon2 with a random salt.
    fn hash_password(password: &str) -> Result<String> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| crate::error::PokerError::password_hash(e.to_string()))?
            .to_string();
        Ok(hash)
    }

    /// Executes a function within a database transaction.
    ///
    /// The transaction is committed if the function returns Ok,
    /// and rolled back if it returns an error.
    ///
    /// # Arguments
    ///
    /// * `f` - Async function that takes a mutable transaction reference
    ///
    /// # Example
    ///
    /// ```ignore
    /// db.transaction(|tx| Box::pin(async move {
    ///     sqlx::query("UPDATE players SET chips = chips + ? WHERE id = ?")
    ///         .bind(50)
    ///         .bind(player_id)
    ///         .execute(&mut **tx)
    ///         .await?;
    ///     Ok(())
    /// })).await
    /// ```
    pub async fn transaction<F, R>(&self, f: F) -> Result<R>
    where
        F: for<'tx> FnOnce(
            &'tx mut sqlx::Transaction<'_, sqlx::Sqlite>,
        ) -> futures::future::BoxFuture<'tx, Result<R>>,
    {
        let mut tx = self.pool.begin().await?;
        let result = f(&mut tx).await;
        match result {
            Ok(r) => {
                tx.commit().await?;
                Ok(r)
            }
            Err(e) => {
                tx.rollback().await?;
                Err(e)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    async fn setup_test_db() -> Database {
        let db = Database::new("sqlite::memory:", 100, 5).await.unwrap();
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

    #[tokio::test]
    async fn test_db_connection_pool_config() {
        let db = Database::new("sqlite::memory:", 100, 5).await.unwrap();
        db.initialize_schema().await.unwrap();
        let player_id = db.create_player("pooluser", "Password123").await.unwrap();
        let player = db.get_player_by_id(player_id).await.unwrap();
        assert!(player.is_some());
    }
}
