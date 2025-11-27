use std::io;
use axum::body::Bytes;
use uuid::Uuid;

pub async fn save_file_to_disk(data: Bytes) -> Result<String, io::Error> {
    let uuid = Uuid::new_v4();
    let file_path = format!("test_data/{}", uuid);

    tokio::fs::write(file_path, &data).await?;
    
    Ok(uuid.to_string())
}