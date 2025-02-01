mod types;

use actix_multipart::Multipart;
use actix_web::{post, App, Error, HttpResponse, HttpServer};
use env_logger::{self, Env};
use futures::StreamExt;
use log::error;
use serde::Serialize;
use serde_json;
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};
use types::{Actor, GithubActions};
use zip::ZipArchive;

const JSON_DIR: &str = "./tmp/";
const OUT_FILE: &str = "actors.json";
const UPLOADED_FILE: &str = "uploaded.zip";

fn process_json_file(file_path: &Path) -> Result<Vec<Actor>, Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    // Use serde_json to parse the entire file as a JSON array or stream
    let records: Vec<GithubActions> = serde_json::from_reader(reader)?;
    let actors: Vec<Actor> = records
        .into_iter()
        .filter_map(|record| record.actor)
        .collect();

    Ok(actors)
}

fn process_json_dir() -> Result<(), Box<dyn std::error::Error>> {
    //info!("Starting processing the json files");
    let nested_actors: Vec<Vec<Actor>> = std::fs::read_dir(Path::new(JSON_DIR))?
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().file_name().and_then(|s| s.to_str()) != Some("actors.json"))
        .filter(|entry| entry.path().extension().and_then(|s| s.to_str()) == Some("json"))
        .map(|entry| process_json_file(&entry.path()))
        .collect::<Result<Vec<_>, _>>()?;
    let actors: Vec<Actor> = nested_actors.into_iter().flatten().collect();

    let path = PathBuf::from(JSON_DIR.to_owned() + OUT_FILE);
    let actors_json = serde_json::to_string(&actors)?;
    let file = File::create(path)?;

    let mut writer = BufWriter::new(file);
    writer.write_all(actors_json.as_bytes())?;
    writer.flush()?;

    Ok(())
}

fn validate_and_uncompress_zip(file_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    // Check if the file exists and is readable
    if !file_path.exists() {
        return Err("File does not exist".into());
    }
    if !file_path.is_file() {
        return Err("Path is not a file".into());
    }

    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut archive = ZipArchive::new(reader)?;

    // Check if the ZIP archive is empty
    if archive.is_empty() {
        return Err("ZIP archive is empty".into());
    }

    // Uncompress files
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        if file.size() == 0 {
            return Err(format!("File {} is corrupted", file.name()).into());
        }
        let outpath = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };

        if file.name().ends_with('/') {
            // Create directories
            std::fs::create_dir_all(&outpath)?;
        } else {
            // Create the output file
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    std::fs::create_dir_all(p)?;
                }
            }
            let tmp = PathBuf::from("./tmp/");
            let mut outfile = File::create(tmp.join(&outpath))?;
            std::io::copy(&mut file, &mut outfile)?;
        }
    }

    Ok(())
}

#[post("/upload")]
async fn upload_zip(mut payload: Multipart) -> Result<HttpResponse, Error> {
    // Create a file to save the uploaded ZIP
    let file_path = PathBuf::from(JSON_DIR.to_owned() + UPLOADED_FILE);

    // Ensure the directory exists
    std::fs::create_dir_all(JSON_DIR)?;

    // Open file with explicit write permissions
    let file = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&file_path)?;

    let mut writer = BufWriter::new(file);

    //info!("Starting to receive the uploaded ZIP file at: {}", file_path.display());

    let mut field_count = 0;

    // Process each field in the multipart payload
    while let Some(field_result) = payload.next().await {
        field_count += 1;

        let mut field = field_result.map_err(|e| {
            error!("Field processing error: {}", e);
            actix_web::error::ErrorBadRequest(format!("Field processing error: {}", e))
        })?;

        // Get field information
        let content_disposition = field.content_disposition();
        let field_name = content_disposition.get_name().unwrap_or("UNKNOWN");

        // Only process fields with the name "file"
        if field_name == "file" {
            // Process each chunk in the field
            while let Some(chunk_result) = field.next().await {
                let chunk = chunk_result.map_err(|e| {
                    error!("Chunk processing error in field #{}: {}", field_count, e);
                    actix_web::error::ErrorBadRequest(format!("Chunk processing error: {}", e))
                })?;

                // Write chunk
                writer.write_all(&chunk).map_err(|e| {
                    error!("Error writing chunk: {}", e);
                    actix_web::error::ErrorInternalServerError(format!("Write error: {}", e))
                })?;
            }
            // Break after processing the first "file" field
            break;
        }
    }

    // Ensure all data is written to the file
    writer.flush()?;

    // Verify file size
    let file_size = std::fs::metadata(&file_path)?.len();

    // Sanity checks
    if file_size == 0 {
        error!("Uploaded file is empty!");
        return Err(actix_web::error::ErrorBadRequest("Uploaded file is empty"));
    }

    match validate_and_uncompress_zip(&file_path) {
        Ok(_) => {
            process_json_dir()?;

            Ok(HttpResponse::Ok().json({
                "ok";
                ""
            }))
        }
        Err(e) => {
            error!("Error processing ZIP file: {}", e);
            Ok(HttpResponse::BadRequest().json({}))
        }
    }
}

#[derive(Debug, Serialize)]
struct ProcessingStats {
    total_records: usize,
    processed_records: usize,
    error_records: usize,
    processing_time: Duration,
    output_file: PathBuf,
}

fn process_large_json_stream(
    file_path: &Path,
    chunk_size: usize,
    max_file_size_mb: usize,
) -> Result<ProcessingStats, Box<dyn std::error::Error>> {
    // Validate file size before processing
    let metadata = std::fs::metadata(file_path)?;
    let file_size_mb = metadata.len() as usize / (1024 * 1024);

    if file_size_mb > max_file_size_mb {
        return Err(format!(
            "File too large. Max allowed: {}MB, Current: {}MB",
            max_file_size_mb, file_size_mb
        )
        .into());
    }

    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let stream = serde_json::Deserializer::from_reader(reader).into_iter::<GithubActions>();

    let start_time = Instant::now();
    let mut processed_records = 0;
    let mut error_records = 0;
    let mut actors = Vec::with_capacity(chunk_size);
    let output_path = PathBuf::from(JSON_DIR.to_owned() + "streamed_actors.json");
    let mut output_file = File::create(&output_path)?;

    for (index, record_result) in stream.enumerate() {
        match record_result {
            Ok(record) => {
                // Process record
                if let Some(actor) = record.actor {
                    actors.push(actor);
                    processed_records += 1;
                }

                // Batch processing
                if actors.len() >= chunk_size {
                    let batch_json = serde_json::to_string(&actors)?;
                    output_file.write_all(batch_json.as_bytes())?;
                    output_file.write_all(b"\n")?; // Newline for batch separation
                    actors.clear();
                }
            }
            Err(e) => {
                error_records += 1;
                error!("Error parsing record at index {}: {}", index, e);
            }
        }
    }

    // Process any remaining records
    if !actors.is_empty() {
        let batch_json = serde_json::to_string(&actors)?;
        output_file.write_all(batch_json.as_bytes())?;
    }

    let duration = start_time.elapsed();

    Ok(ProcessingStats {
        total_records: processed_records + error_records,
        processed_records,
        error_records,
        processing_time: duration,
        output_file: output_path,
    })
}

#[post("/upload_large")]
async fn upload_large_json(mut payload: Multipart) -> Result<HttpResponse, Error> {
    let file_path = PathBuf::from(JSON_DIR.to_owned() + UPLOADED_FILE);

    // Ensure directory exists
    std::fs::create_dir_all(JSON_DIR)?;

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

    // Process large file with streaming
    match process_large_json_stream(&file_path, 1000, 500) {
        Ok(stats) => Ok(HttpResponse::Ok().json(stats)),
        Err(e) => {
            error!("Error processing large file: {}", e);
            Ok(HttpResponse::BadRequest().body(format!("Processing error: {}", e)))
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Configure logging
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();

    HttpServer::new(|| {
        App::new()
            // .wrap(TracingLogger::default())
            .service(upload_zip)
            .service(upload_large_json)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
