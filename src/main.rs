use axum::extract::DefaultBodyLimit;
use axum::http::Method;
use axum::Router;
use axum::routing::{get, post};
use tower_http::cors::{Any, CorsLayer};
use crate::api::handlers::{download_file, upload_file};

mod api;
mod services;

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(Any);

    let router = Router::new()
        .route("/upload", post(upload_file))
        .route("/download/{file_id}", get(download_file))
        .layer(DefaultBodyLimit::max(20 * 1024 * 1024))
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await.unwrap();

    axum::serve(listener, router).await.unwrap()
}
