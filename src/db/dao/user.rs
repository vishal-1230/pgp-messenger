use crate::db::entities::user::UserEntity;

pub struct UserDao {
    pub db: crate::db::connection::Database
}

impl UserDao {

    pub async fn create_user_table(&self) -> Result<(), sqlx::Error> {
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS users (
                id INT PRIMARY KEY AUTO_INCREMENT,
                username VARCHAR(50) NOT NULL,
                email VARCHAR(150) NOT NULL,
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

    pub async fn create_user(&self, username: &str, email: &str, password_hash: &str, pub_pgp_key: &str) -> Result<UserEntity, sqlx::Error> {
        let result = sqlx::query(
            "INSERT INTO users (username, email, password_hash, pub_pgp_key) VALUES (?, ?, ?, ?)"
        )
        .bind(username)
        .bind(email)
        .bind(password_hash)
        .bind(pub_pgp_key)
        .execute(&self.db.pool)
        .await?;

        Ok(UserEntity {
            _id: result.last_insert_id(),
            _username: username.to_string(),
            _email: email.to_string(),
            _password_hash: password_hash.to_string(),
            _pub_pgp_key: pub_pgp_key.to_string(),
            _created_at: chrono::Utc::now(),
            _updated_at: chrono::Utc::now(),
        })
    }
}