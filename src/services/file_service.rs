use std::io;
use axum::body::Bytes;
use uuid::Uuid;

pub async fn save_file_to_disk(data: Bytes) -> Result<String, io::Error> {
    let uuid = Uuid::new_v4();
    let file_path = format!("test_data/{}", uuid);

    tokio::fs::write(file_path, &data).await?;
    
    Ok(uuid.to_string())
}

pub async fn get_file_from_disk(file_id: String) -> Result<Vec<u8>, io::Error> {
    let file_path = format!("test_data/{}", file_id);
    
    tokio::fs::read(file_path).await
}