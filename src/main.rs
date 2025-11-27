use axum::extract::DefaultBodyLimit;
use axum::Router;
use axum::routing::{post};
use crate::api::handlers::{upload_file};

mod api;

#[tokio::main]
async fn main() {
    let router = Router::new()
        .route("/upload", post(upload_file))
        .layer(DefaultBodyLimit::max(20 * 1024 * 1024));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await.unwrap();

    axum::serve(listener, router).await.unwrap()
}
