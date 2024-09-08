mod user;
mod spark;
mod db;
mod jwt;
mod config;

use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use std::sync::Arc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables from .env file
    dotenv::dotenv().ok();

    // Check if the MongoDB connection is successful
    let db = db::check_database_connection().await.expect("Failed to connect to MongoDB");

    println!("Connected to MongoDB database: {}", *config::MONGO_DB_NAME);

    // Start the web server
    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db::AppState {
                db: Arc::new(db.clone()),
            }))
            .wrap(
                Cors::default()
                    .allow_any_origin() // 允许所有来源，或者使用 .allowed_origin("http://localhost:3000") 限制特定来源
                    .allow_any_method()  // 允许所有 HTTP 方法
                    .allow_any_header()  // 允许所有请求头
            )
            .route("/register", web::post().to(user::register_user))
            .route("/login", web::post().to(user::login_user))
            .route("/sparks/latest", web::get().to(spark::get_sparks_by_date))
            .route("/sparks/top", web::get().to(spark::get_sparks_by_star))
            .route("/sparks", web::post().to(spark::add_spark))
            .route("/sparks/{id}/like", web::post().to(spark::like_spark))
    })
    .bind("0.0.0.0:8080")?;

    println!("Web server is running on http://0.0.0.0:8080");
    server.run().await
}