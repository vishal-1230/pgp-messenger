use std::env::var as env_var;

pub struct AppConfig {
    pub db_host: String,
    pub db_user: String,
    pub db_password: String,
    pub db_name: String,
}

impl AppConfig {
    pub fn new() -> Self {
        AppConfig {
            db_host: env_var("DB_HOST").unwrap_or("127.0.0.1".to_string()),
            db_user: env_var("DB_USER").unwrap_or("root".to_string()),
            db_password: env_var("DB_PASSWORD").unwrap_or("".to_string()),
            db_name: env_var("DB_NAME").unwrap_or("testrust".to_string()),
        }
    }
}