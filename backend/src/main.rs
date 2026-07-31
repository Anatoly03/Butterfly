//! The entry point of the Butterfly backend application. This module starts the
//! [axum] web server.

use axum::{Router, routing::get};
use sqlx::{SqlitePool, sqlite::SqliteConnectOptions};
use std::str::FromStr;

/// The entry point of the Butterfly backend application. This function starts the
/// [axum] web server.
#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    std::fs::create_dir_all(".butterfly").unwrap();

    // build our application with a single route
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    // run our app with hyper, listening globally on specified port
    let port = dotenv::var("PORT").unwrap_or("3000".to_string());
    let hostname = format!("0.0.0.0:{port}");
    let listener = tokio::net::TcpListener::bind(&hostname).await.unwrap();
    println!("Listening on {hostname}");

    // production ready sqlx pool configuration
    let options = SqliteConnectOptions::from_str("sqlite://.butterfly/data.db")
        .unwrap()
        .create_if_missing(true);
    let _pool = SqlitePool::connect_with(options).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
