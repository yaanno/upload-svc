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
) -> Result<HttpResponse, Error> {
    let file_path = PathBuf::from(config.json_dir.to_owned() + &config.upload_file_name);
    std::fs::create_dir_all(&config.json_dir)?;

    let file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&file_path)?;

    file_processing::save_multipart_file(payload, file)
        .await
        .map_err(actix_web::error::ErrorBadRequest)?;

    match file_processing::validate_and_uncompress_zip(&config, &file_path).await {
        Ok(_) => {
            process_fn(web::Data::new(AppConfig::default()))
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
    handle_upload(config, payload, processing::process_json_dir).await
}

#[post("/upload_large")]
pub async fn upload_large_zip(
    config: web::Data<config::AppConfig>,
    payload: Multipart,
) -> Result<HttpResponse, Error> {
    handle_upload(config, payload, processing::process_large_json_dir).await
}