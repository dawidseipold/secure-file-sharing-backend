use crate::models::PublicKeyRecord;
use axum::body::Bytes;
use anyhow::Context;
use surrealdb::{Surreal};
use surrealdb::engine::local::Db;
use uuid::Uuid;

pub async fn save_key(key: Bytes, db: &Surreal<Db>) -> Result<String, anyhow::Error> {
    let user_id = Uuid::new_v4();
    let public_key = String::from_utf8(key.to_vec())
        .context("Invalid UTF-8 in key")?;

    let record = PublicKeyRecord {
        id: None,
        public_key,
        created_at: chrono::Utc::now().to_rfc3339(),
    };

    let _: Option<PublicKeyRecord> = db.create(("keys", user_id.to_string()))
        .content(record)
        .await?;

    Ok(user_id.to_string())
}
pub async fn get_key(
    user_id: String,
    db: &Surreal<Db>,
) -> Result<Option<PublicKeyRecord>, anyhow::Error> {
    let record: Option<PublicKeyRecord> = db
        .select(("keys", user_id))
        .await?;

    Ok(record)
}
