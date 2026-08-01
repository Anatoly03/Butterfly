//! The entry point of the Butterfly backend application. This module starts the
//! [axum] web server.

mod database;

use axum::{Router, routing::get};

/// The entry point of the Butterfly backend application. This function starts the
/// [axum] web server.
#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    std::fs::create_dir_all(".butterfly").unwrap();

    // create sqlite pool and migrate
    let pool = database::init().await.unwrap();
    if dotenv::var("DATABASE_AUTOMIGRATE").map_or(false, |k| k != "0" && k != "false") {
        let _ = database::migrate(&pool).await.unwrap();
    }

    // build our application with a single route
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    // run our app with hyper, listening globally on specified port
    let port = dotenv::var("PORT").unwrap_or("3000".to_string());
    let hostname = format!("0.0.0.0:{port}");
    let listener = tokio::net::TcpListener::bind(&hostname).await.unwrap();
    println!("Listening on {hostname}");

    axum::serve(listener, app).await.unwrap();
}
