use std::time::Duration;
use chrono::{DateTime, Utc};
use surrealdb::engine::local::Db;
use surrealdb::Surreal;
use crate::models::FileRecord;

pub async fn start_cleaner(db: Surreal<Db>) {
    loop {
        tokio::time::sleep(Duration::from_secs(60)).await;
        println!("[Cleaner] Searching for expired files...");

        let files: Vec<FileRecord> = match db.select("files").await {
            Ok(f) => f,
            Err(e) => {
                eprintln!("[CLEANER] Error while downloading files: {}", e);
                continue;
            }
        };

        let now = Utc::now();

        for file in files {
            let created_at = match DateTime::parse_from_rfc3339(&file.created_at) {
                Ok(dt) => dt.with_timezone(&Utc),
                Err(_) => continue
            };

            let duration = match file.expiration.as_str() {
                "1_hour" => chrono::Duration::hours(1),
                "1_day" => chrono::Duration::days(1),
                "7_days" => chrono::Duration::days(7),
                "1_month" => chrono::Duration::days(30),
                "burn_on_read" => chrono::Duration::days(365),
                _ => chrono::Duration::days(7),
            };

            let deadline = created_at + duration;

            if now > deadline {
                println!("[CLEANER] Deleting expired file: {}", file.filename);

                if let Err(e) = tokio::fs::remove_file(&file.file_path).await {
                    eprintln!("Error while removing a file: {}", e);
                }

                if let Some(thing) = &file.id {
                    let _: Result<Option<FileRecord>, _> = db.delete((thing.tb.clone(), thing.id.to_string())).await;
                }
            }
        }
    }
}