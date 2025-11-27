use axum::extract::{Multipart, Path};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use serde::Serialize;
use crate::services::file_service::save_file_to_disk;

#[derive(Serialize)]
pub struct UploadResponse {
    pub file_id: String
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String
}


pub async fn upload_file(mut multipart: Multipart) -> Result<Json<UploadResponse>, (StatusCode, Json<ErrorResponse>)> {
    while let Some(field) = multipart.next_field().await.map_err(|e|
        (StatusCode::BAD_REQUEST, Json(ErrorResponse { error: e.to_string() }))
    )? {
        let data = field.bytes().await.map_err(|e|
            (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: format!("Failed to read bytes: {}", e) }))
        )?;

        let file_id = save_file_to_disk(data).await.map_err(|e|
            (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: format!("Failed to save file: {}", e) }))
        )?;

        return Ok(Json(UploadResponse { file_id }));
    }

    Err((StatusCode::BAD_REQUEST, Json(ErrorResponse { error: "No file provided in the request".to_string() })))
}

pub async fn download_file(Path(file_id): Path<String>) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    let file = tokio::fs::read(format!("test_data/{}", file_id)).await.map_err(|e|
        (StatusCode::NOT_FOUND, Json(ErrorResponse { error: format!("Failed to read file: {}", e) }))
    )?;

     Ok(file)
}