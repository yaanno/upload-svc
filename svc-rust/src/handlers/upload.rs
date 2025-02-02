use std::path::PathBuf;

use crate::config::{self, AppConfig};
use crate::handlers::processing;
use crate::utils::file_processing;
use actix_multipart::Multipart;
use actix_web::{post, web, Error, HttpResponse};
use serde_json::json;

async fn handle_upload(
    config: web::Data<AppConfig>,
    payload: Multipart,
    process_fn: impl Fn(web::Data<AppConfig>) -> Result<(), Box<dyn std::error::Error>>,
    is_large_upload: bool,
) -> Result<HttpResponse, Error> {
    // Choose appropriate directories based on upload type
    let (upload_dir, json_dir) = if is_large_upload {
        (&config.large_upload_dir, &config.large_json_dir)
    } else {
        (&config.upload_dir, &config.json_dir)
    };

    // Create directories if they don't exist
    std::fs::create_dir_all(upload_dir)?;
    std::fs::create_dir_all(json_dir)?;

    // Create file path using the chosen upload directory
    let file_path = PathBuf::from(upload_dir.to_owned() + &config.upload_file_name);

    let file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&file_path)?;

    file_processing::save_multipart_file(payload, file)
        .await
        .map_err(actix_web::error::ErrorBadRequest)?;

    // Create a new config with the appropriate directories for processing
    let processing_config = AppConfig {
        upload_dir: upload_dir.clone(),
        json_dir: json_dir.clone(),
        ..config.as_ref().clone()
    };

    match file_processing::validate_and_uncompress_zip(&processing_config, &file_path).await {
        Ok(_) => {
            process_fn(web::Data::new(processing_config))
                .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;
            Ok(HttpResponse::Ok().json(json!({"status": "ok"})))
        }
        Err(e) => {
            tracing::error!("Zip validation error: {}", e);
            Ok(HttpResponse::BadRequest().json(json!({
                "error": e.to_string()
            })))
        }
    }
}

#[post("/upload")]
pub async fn upload_zip(
    config: web::Data<AppConfig>,
    payload: Multipart,
) -> Result<HttpResponse, Error> {
    handle_upload(config, payload, processing::process_json_dir, false).await
}

#[post("/upload_large")]
pub async fn upload_large_zip(
    config: web::Data<AppConfig>,
    payload: Multipart,
) -> Result<HttpResponse, Error> {
    handle_upload(config, payload, processing::process_large_json_dir, true).await
}