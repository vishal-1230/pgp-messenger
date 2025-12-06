use crate::db::entities::user::UserEntity;
use sqlx::Row;

pub struct UserDao {
    pub db: crate::db::connection::Database
}

impl UserDao {

    pub async fn create_user_table(&self) -> Result<(), sqlx::Error> {
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS users (
                id INT PRIMARY KEY AUTO_INCREMENT,
                username VARCHAR(50) NOT NULL UNIQUE,
                password_hash VARCHAR(255) NOT NULL,
                pub_pgp_key TEXT,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
            )"
        )
        .execute(&self.db.pool)
        .await?;
        Ok(())
    }

    pub async fn create_user(&self, username: &str, password_hash: &str, pub_pgp_key: &str) -> Result<UserEntity, sqlx::Error> {
        // Insert the user first
        sqlx::query(
            "INSERT INTO users (username, password_hash, pub_pgp_key) VALUES (?, ?, ?)"
        )
        .bind(username)
        .bind(password_hash)
        .bind(pub_pgp_key)
        .execute(&self.db.pool)
        .await?;

        // Fetch the newly inserted user using the username
        let row = sqlx::query_as::<_, UserEntity>(
            "SELECT id, username, password_hash, pub_pgp_key, created_at, updated_at FROM users WHERE username = ?"
        )
        .bind(username)
        .fetch_one(&self.db.pool)
        .await?;
        Ok(row)
    }

    pub async fn is_username_available(&self, username: &str) -> Result<bool, sqlx::Error> {
        let row = sqlx::query("SELECT 1 FROM users WHERE username = ? LIMIT 1")
            .bind(username)
            .fetch_optional(&self.db.pool)
            .await?;
        Ok(row.is_none())
    }

    pub async fn get_user_by_username(&self, username: &str) -> Result<Option<UserEntity>, sqlx::Error> {

        if let Some(row) = sqlx::query_as::<_, UserEntity>(
            "SELECT id, username, password_hash, pub_pgp_key, created_at, updated_at FROM users WHERE username = ? LIMIT 1"
        )
        .bind(username)
        .fetch_optional(&self.db.pool)
        .await? {
            Ok(Some(row))
        } else {
            Ok(None)
        }
    }
}