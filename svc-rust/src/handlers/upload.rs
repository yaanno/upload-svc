use std::io::{BufWriter, Write};
use std::path::PathBuf;

use actix_web::{web, HttpResponse, Error, post};
use actix_multipart::Multipart;
use futures::StreamExt;
use tracing::error;
use crate::config::{self, AppConfig};
use crate::utils::file_processing;
use crate::handlers::processing;
use crate::utils::json_processing::{process_large_json_stream, ProcessingStats};

#[post("/upload")]
pub async fn upload_zip(config: web::Data<AppConfig>, payload: Multipart) -> Result<HttpResponse, Error> {
    let file_path = PathBuf::from(config.json_dir.to_owned() + &config.upload_file_name);
    std::fs::create_dir_all(&config.json_dir)?;

    let file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&file_path)?;

    // Existing upload logic, moved to a utility function
    file_processing::save_multipart_file(payload, file).await.map_err(|e| actix_web::error::ErrorBadRequest(e))?;

    match file_processing::validate_and_uncompress_zip(&file_path).await {
        Ok(_) => {
            processing::process_json_dir(web::Data::new(AppConfig::default()))?;
            Ok(HttpResponse::Ok().json("ok"))
        }
        Err(e) => {
            log::error!("Zip validation error: {}", e);
            Ok(HttpResponse::BadRequest().json({"error"; e.to_string()}))
        }
    }
}

#[post("/upload_large")]
async fn upload_large_zip(config: web::Data<config::AppConfig>, mut payload: Multipart) -> Result<HttpResponse, Error> {
    let file_path = PathBuf::from(config.json_dir.to_owned() + config.upload_file_name.as_str());
    let unzipped_dir = config.json_dir.to_owned() + "unzipped/";

    // Ensure directories exist
    std::fs::create_dir_all(&config.json_dir)?;
    std::fs::create_dir_all(&unzipped_dir)?;

    // Open file with explicit write permissions
    let file = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&file_path)?;

    let mut writer = BufWriter::new(file);

    // Process multipart upload
    while let Some(field_result) = payload.next().await {
        let mut field = field_result.map_err(|e| {
            error!("Field processing error: {}", e);
            actix_web::error::ErrorBadRequest(format!("Field processing error: {}", e))
        })?;

        // Only process "file" fields
        let content_disposition = field.content_disposition();
        let field_name = content_disposition.get_name().unwrap_or("UNKNOWN");

        if field_name == "file" {
            while let Some(chunk_result) = field.next().await {
                let chunk = chunk_result.map_err(|e| {
                    error!("Chunk processing error: {}", e);
                    actix_web::error::ErrorBadRequest(format!("Chunk processing error: {}", e))
                })?;

                writer.write_all(&chunk)?;
            }
            break; // Process only first file field
        }
    }

    writer.flush()?;

    // Unzip the file
    let unzip_output = std::process::Command::new("unzip")
        .arg("-o")  // overwrite existing files
        .arg(&file_path)
        .arg("-d")
        .arg(&unzipped_dir)
        .output()
        .map_err(|e| {
            error!("Unzip command failed: {}", e);
            actix_web::error::ErrorInternalServerError("Failed to unzip file")
        })?;

    if !unzip_output.status.success() {
        let error_message = String::from_utf8_lossy(&unzip_output.stderr);
        error!("Unzip error: {}", error_message);
        return Err(actix_web::error::ErrorBadRequest(format!("Unzip failed: {}", error_message)));
    }

    // Process each JSON file in the unzipped directory
    let mut total_stats = ProcessingStats {
        total_records: 0,
        processed_records: 0,
        error_records: 0,
        processing_time: std::time::Duration::new(0, 0),
        output_file: PathBuf::new(),
    };

    for entry in std::fs::read_dir(&unzipped_dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.extension().map_or(false, |ext| ext == "json") {
            match process_large_json_stream(&config, &path, 1000, 500) {
                Ok(stats) => {
                    total_stats.total_records += stats.total_records;
                    total_stats.processed_records += stats.processed_records;
                    total_stats.error_records += stats.error_records;
                    total_stats.processing_time += stats.processing_time;
                },
                Err(e) => {
                    error!("Error processing JSON file {}: {}", path.display(), e);
                    total_stats.error_records += 1;
                }
            }
        }
    }

    Ok(HttpResponse::Ok().json(total_stats))
}