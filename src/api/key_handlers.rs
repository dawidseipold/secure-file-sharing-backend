use axum::body::Bytes;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::Json;
use serde::Serialize;
use crate::services::key_service::{get_key_from_disk, save_key_to_disk};

#[derive(Serialize)]
pub struct PublishResponse {
    pub user_id: String
}

#[derive(Serialize)]
pub struct KeyResponse {
    pub key: String,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String
}

pub async fn publish_key(body: Bytes) -> Result<Json<PublishResponse>, (StatusCode, Json<ErrorResponse>)> {
    if body.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse { error: "Request body cannot be empty".to_string() })
        ));
    }

    let user_id = save_key_to_disk(body).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { error: format!("Failed to save key: {}", e) })
        )
    })?;

    Ok(Json(PublishResponse { user_id }))
}

pub async fn get_key(Path(user_id): Path<String>) -> Result<Json<KeyResponse>, (StatusCode, Json<ErrorResponse>)> {
    let key_content = get_key_from_disk(user_id).await.map_err(|e| {
        (
            StatusCode::NOT_FOUND,
            Json(ErrorResponse { error: format!("Failed to read key: {}", e) })
        )
    })?;

    Ok(Json(KeyResponse { key: key_content }))
}