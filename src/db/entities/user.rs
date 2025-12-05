#[derive(Debug)]
pub struct UserEntity {
    pub _id: u64,
    pub _username: String,
    pub _email: String,
    pub _password_hash: String,
    pub _pub_pgp_key: String,
    pub _created_at: chrono::DateTime<chrono::Utc>,
    pub _updated_at: chrono::DateTime<chrono::Utc>,
}