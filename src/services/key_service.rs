use std::io;
use axum::body::Bytes;
use uuid::Uuid;

pub async fn save_key_to_disk(key: Bytes) -> Result<String, io::Error> {
    let user_id = Uuid::new_v4();
    let file_path = format!("keys_data/{}", user_id);

    tokio::fs::write(file_path, &key).await?;

    Ok(user_id.to_string())
}
pub async fn get_key_from_disk(user_id: String) -> Result<String, io::Error> {
    let file_path = format!("keys_data/{}", user_id);
    let key_content = tokio::fs::read_to_string(file_path).await?;

    Ok(key_content)
}