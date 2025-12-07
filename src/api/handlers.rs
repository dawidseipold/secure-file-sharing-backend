use crate::AppState;
use crate::models::{FileDownloadDto, FileRecord, MetadataDto};
use crate::services::file_service;
use axum::Json;
use axum::extract::{Multipart, Path, State};
use axum::http::StatusCode;
use serde::{Serialize};
use uuid::Uuid;

#[derive(Serialize)]
pub struct UploadResponse {
    pub file_id: String,
}

#[derive(Serialize)]
pub struct FileResponse {
    pub file_content: String,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

pub async fn upload_file(
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> Result<Json<UploadResponse>, (StatusCode, Json<ErrorResponse>)> {
    let file_id = Uuid::new_v4();
    let file_id_str = file_id.to_string();

    let mut parsed_metadata: Option<MetadataDto> = None;
    let mut saved_file_path: Option<String> = None;

    while let Some(field) = multipart.next_field().await.map_err(|e| {
        (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: e.to_string(),
            }),
        )
    })? {
        let name = field.name().unwrap_or("").to_string();

        if name == "metadata" {
            let data = field.bytes().await.map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorResponse {
                        error: format!("Read meta error: {}", e),
                    }),
                )
            })?;

            let dto: MetadataDto = serde_json::from_slice(&data).map_err(|e| {
                (
                    StatusCode::BAD_REQUEST,
                    Json(ErrorResponse {
                        error: format!("Invalid JSON metadata: {}", e),
                    }),
                )
            })?;

            parsed_metadata = Some(dto);
        } else if name == "file" {
            let data = field.bytes().await.map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorResponse {
                        error: format!("Read file error: {}", e),
                    }),
                )
            })?;

            let path = file_service::save_file_blob(&file_id_str, data)
                .await
                .map_err(|e| {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(ErrorResponse {
                            error: format!("Save file error: {}", e),
                        }),
                    )
                })?;

            saved_file_path = Some(path);
        }
    }

    if let (Some(metadata), Some(path)) = (parsed_metadata, saved_file_path) {
        file_service::create_file_record(&state.db, &file_id_str, metadata, path)
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorResponse {
                        error: format!("DB Error: {}", e),
                    }),
                )
            })?;

        Ok(Json(UploadResponse {
            file_id: file_id_str,
        }))
    } else {
        Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "Missing 'metadata' or 'file' field".to_string(),
            }),
        ))
    }
}

pub async fn download_file(
    State(state): State<AppState>,
    Path(file_id): Path<String>,
) -> Result<Json<FileDownloadDto>, (StatusCode, Json<ErrorResponse>)> {
    match file_service::get_file_package(&state.db, file_id).await {
        Ok(dto) => Ok(Json(dto)),
        Err(e) => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: format!("Failed to download file package: {}", e)
            })
        ))
    }
}

pub async fn list_user_files(
    State(state): State<AppState>,
    Path(user_id): Path<String>
) -> Result<Json<Vec<FileRecord>>, (StatusCode, Json<ErrorResponse>)> {

    match file_service::list_files_for_user(&state.db, user_id).await {
        Ok(files) => Ok(Json(files)),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { error: format!("DB Error: {}", e) })
        ))
    }
}