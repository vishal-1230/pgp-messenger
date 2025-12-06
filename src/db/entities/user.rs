#![allow(dead_code)]
#[derive(sqlx::FromRow)]
#[derive(Debug)]
pub struct UserEntity {
    pub id: i32,
    pub username: String,
    pub password_hash: String,
    pub pub_pgp_key: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}