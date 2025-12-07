use axum::body::Bytes;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use serde::Serialize;
use crate::AppState;
use crate::services::key_service;


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

pub async fn publish_key(State(state): State<AppState>, body: Bytes) -> Result<Json<PublishResponse>, (StatusCode, Json<ErrorResponse>)> {
    if body.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse { error: "Request body cannot be empty".to_string() })
        ));
    }

    let user_id = key_service::save_key(body, &state.db).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { error: format!("Failed to save key: {}", e) })
        )
    })?;

    Ok(Json(PublishResponse { user_id }))
}

pub async fn get_key(State(state): State<AppState>, Path(user_id): Path<String>) -> Result<Json<KeyResponse>, (StatusCode, Json<ErrorResponse>)> {
    match key_service::get_key(user_id, &state.db).await {
        Ok(Some(record)) => {
            let response = KeyResponse {
                key: record.public_key,
            };

            Ok(Json(response))
        },
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse { error: "Key not found".to_string() })
        )),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: format!("Failed to read key: {}", e)
            })
        ))
    }
}