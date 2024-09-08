use mongodb::{bson::doc, Client, Database};
use std::sync::Arc;
use crate::config;


#[derive(Clone)]
pub struct AppState {
    pub db: Arc<Database>,
}
// pub static MONGO_DB_NAME: &str = "your_db_name"; // 替换为您的数据库名称

pub async fn get_database() -> Database {
    let mongo_uri_copy = (*config::MONGO_URI).clone();
    let client = Client::with_uri_str(mongo_uri_copy).await.expect("Failed to initialize MongoDB client");
    client.database(&config::MONGO_DB_NAME)
}

pub async fn check_database_connection() -> Result<Database, mongodb::error::Error> {
    let db = get_database().await;
    db.run_command(doc! {"ping": 1}, None).await?;
    Ok(db)
}