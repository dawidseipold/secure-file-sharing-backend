use crate::models::{FileDownloadDto, FileRecord, MetadataDto};
use anyhow::{Context, Result};
use axum::body::Bytes;
use std::io;
use surrealdb::Surreal;
use surrealdb::engine::local::Db;

pub async fn save_file_blob(file_id: &str, data: Bytes) -> Result<String, anyhow::Error> {
    let file_path = format!("test_data/{}.bin", file_id);
    tokio::fs::write(&file_path, &data).await?;

    Ok(file_path)
}

pub async fn create_file_record(
    db: &Surreal<Db>,
    file_id: &str,
    metadata: MetadataDto,
    saved_path: String,
) -> Result<()> {
    let record = FileRecord {
        id: None,
        sender_id: metadata.sender_id,
        recipients: metadata.recipients,
        file_iv: metadata.file_iv,
        expiration: metadata.expiration,
        note_iv: metadata.note_iv,
        encrypted_note: metadata.encrypted_note,
        filename: metadata.filename,
        mime_type: metadata.mime_type,
        file_path: saved_path,
        created_at: chrono::Utc::now().to_rfc3339(),
    };

    let _: Option<FileRecord> = db.create(("files", file_id)).content(record).await?;

    Ok(())
}

pub async fn get_file_from_disk(file_id: String) -> Result<Vec<u8>, io::Error> {
    let file_path = format!("test_data/{}.bin", file_id);

    tokio::fs::read(file_path).await
}

pub async fn list_files_for_user(db: &Surreal<Db>, user_id: String) -> Result<Vec<FileRecord>> {
    let sql = "SELECT * FROM files WHERE recipients.user_id CONTAINS $uid ORDER BY created_at DESC";
    let mut response = db.query(sql).bind(("uid", user_id)).await?;

    let files: Vec<FileRecord> = response.take(0)?;

    Ok(files)
}

pub async fn get_file_package(
    db: &Surreal<Db>,
    file_id: String
) -> Result<FileDownloadDto> {
    let record: Option<FileRecord> = db.select(("files", &file_id)).await?;
    let record = record.context("File not found in DB")?;

    let file_bytes = tokio::fs::read(&record.file_path).await?;

    use base64::{Engine as _, engine::general_purpose};
    let encoded_content = general_purpose::STANDARD.encode(&file_bytes);

    let metadata = MetadataDto {
        sender_id: record.sender_id,
        recipients: record.recipients,
        file_iv: record.file_iv,
        expiration: record.expiration,
        note_iv: record.note_iv,
        encrypted_note: record.encrypted_note,
        filename: record.filename,
        mime_type:  record.mime_type,
    };

    Ok(FileDownloadDto {
        file_content: encoded_content,
        metadata
    })
}