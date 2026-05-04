use axum::{
    routing::get,
    routing::post,
    Router,
};
use sqlx::SqlitePool;
use tokio::net::TcpListener;
use dotenvy;
use crate::handlers::admin_login;
mod handlers;
mod models;
#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    
    let connection =
        SqlitePool::connect("sqlite:db/weektech.db?mode=rwc")
            .await.expect("Failed while connecting to sqlite database");

    let app = Router::new()
        .route("/register", get(handler))
        .route("/register", post(handler))
        .route("/admin/login", post(admin_login::admin_login))
        .with_state(connection);

    let listener =
        TcpListener::bind("0.0.0.0:3000")
        .await.expect(
            "Failed while triying to establish port listener");

    axum::serve(listener, app).await.expect(
        "Failed while triying to build server.");

    
}

async fn handler() -> &'static str {
    "ok"
}
