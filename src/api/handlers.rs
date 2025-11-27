use axum::extract::Multipart;

pub async fn upload_file(mut multipart: Multipart) -> &'static str {
    loop {
        let next_field = multipart.next_field().await;

        match next_field {
            Ok(Some(field)) => {
                let name = field.name().unwrap_or("unknown").to_string();
                println!("Received field: {}", name);
            }

            Ok(None) => {
                break;
            }

            Err(e) => {
                println!("Upload Error: {:?}", e);
                return "Error while uploading files"
            }
        }
    }

    "Upload finished"
}