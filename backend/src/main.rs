//! The entry point of the Butterfly backend application. This module starts the
//! [axum] web server.

use axum::{
    routing::get,
    Router,
};

/// The entry point of the Butterfly backend application. This function starts the
/// [axum] web server.
#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    // build our application with a single route
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    // run our app with hyper, listening globally on specified port
    let hostname = format!("0.0.0.0:{}", dotenv::var("PORT").unwrap_or("3000".to_string()));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on {hostname}");

    axum::serve(listener, app).await.unwrap();
}
