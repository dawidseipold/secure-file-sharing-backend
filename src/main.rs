use axum::extract::DefaultBodyLimit;
use axum::http::Method;
use axum::Router;
use axum::routing::{get, post};
use surrealdb::engine::local::Db;
use surrealdb::Surreal;
use tower_http::cors::{Any, CorsLayer};
use crate::api::handlers::{download_file, list_user_files, upload_file};
use crate::api::key_handlers::{get_key, publish_key};
use crate::db::init_db;

mod api;
mod services;
mod db;
mod models;

#[derive(Clone)]
pub struct AppState {
    pub db: Surreal<Db>,
}

#[tokio::main]
async fn main() {
    let db = init_db().await.unwrap();
    let state = AppState { db };

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(Any);

    let router = Router::new()
        .route("/upload", post(upload_file))
        .route("/download/{file_id}", get(download_file))
        .route("/files/user/{user_id}", get(list_user_files))
        .route("/keys", post(publish_key))
        .route("/keys/{user_id}", get(get_key))
        .layer(DefaultBodyLimit::max(20 * 1024 * 1024))
        .layer(cors)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await.unwrap();

    axum::serve(listener, router).await.unwrap()
}
