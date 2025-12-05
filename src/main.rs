use crate::db::{connection::Database, entities::user::UserEntity};

mod config;
mod db;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let user_dao = db::dao::user::UserDao {
        db: get_db_connection().await
    };
    user_dao.create_user_table().await.unwrap();  // todo: handle error
    let created_user: UserEntity = user_dao.create_user("test_user", "test@example.com", "hashed_password", "public_key").await.unwrap();
    println!("Created user: {:?}", created_user);
}

async fn get_db_connection() -> Database {
    let config = config::AppConfig::new();
    let db_conn_url: String = format!(
        "mysql://{}:{}@{}/{}",
        config.db_user, config.db_password, config.db_host, config.db_name
    );
    let result: Result<Database, sqlx::Error> = db::connection::Database::connect(&db_conn_url).await;  // todo: handle error
    match result {
        Err(e) => {
            panic!("Failed to connect to database: {}", e);
        }
        Ok(db_conn) => db_conn,
    }
}