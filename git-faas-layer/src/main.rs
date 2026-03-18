mod error;
mod git_ops;
mod handlers;
mod models;

use axum::{routing::post, Router};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/api/v1/context", post(handlers::extract_handler))
        .route("/api/v1/commit", post(handlers::commit_handler))
        .route("/api/v1/diff", post(handlers::diff_handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .expect("failed to bind port 8080");

    tracing::info!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app)
        .await
        .expect("server error");
}
