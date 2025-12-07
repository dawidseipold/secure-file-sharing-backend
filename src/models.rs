use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Serialize, Deserialize)]
pub struct PublicKeyRecord {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Thing>,
    pub(crate) public_key: String,
    pub(crate) created_at: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RecipientEntry {
    pub user_id: String,
    pub encrypted_key: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FileRecord {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Thing>,

    // Frontend
    pub sender_id: String,
    pub recipients: Vec<RecipientEntry>,
    pub file_iv: String,
    pub expiration: String,
    pub note_iv: Option<String>,
    pub encrypted_note: Option<String>,

    // Backend
    pub file_path: String,
    pub created_at: String
}

#[derive(Deserialize, Serialize)]
pub struct MetadataDto {
    pub sender_id: String,
    pub recipients: Vec<RecipientEntry>,
    pub file_iv: String,
    pub expiration: String,
    pub note_iv: Option<String>,
    pub encrypted_note: Option<String>
}

#[derive(Serialize)]
pub struct FileDownloadDto {
    pub file_content: String, // Base64
    pub metadata: MetadataDto,
}