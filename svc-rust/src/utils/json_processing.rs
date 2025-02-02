use std::{
    fs::File,
    io::{BufReader, Write},
    path::{Path, PathBuf},
    time::{Duration, Instant},
};

use serde::Serialize;
use tracing::error;

use crate::{
    config,
    types::{Actor, Event},
};

#[derive(Debug, Serialize)]
pub struct ProcessingStats {
    pub total_records: usize,
    pub processed_records: usize,
    pub error_records: usize,
    pub processing_time: Duration,
    pub output_file: PathBuf,
}

pub(crate) fn process_json_file(
    file_path: &Path,
) -> Result<Vec<Actor>, Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let _reader = BufReader::new(file);

    // Use serde_json to parse the entire file as a JSON array or stream
    let records: Vec<Event> = serde_json::from_reader(_reader)?;
    let actors: Vec<Actor> = records
        .into_iter()
        .filter_map(|record| record.actor)
        .collect();

    Ok(actors)
}

pub fn process_large_json_stream(
    config: &config::AppConfig,
    file_path: &Path,
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
    let _reader = BufReader::new(file);

    // Read entire file contents to handle no-line-terminator case
    let file_contents = std::fs::read_to_string(file_path)?;
    println!("Total file size: {} bytes", file_contents.len());

    // Try parsing as JSON array first
    if let Ok(records) = serde_json::from_str::<Vec<serde_json::Value>>(&file_contents) {
        let start_time = Instant::now();
        let mut processed_records = 0;
        let mut error_records = 0;
        let output_path = PathBuf::from(config.json_dir.to_owned() + "actors-stream.json");
        let mut output_file = File::create(&output_path)?;

        // Write opening bracket for JSON array
        output_file.write_all(b"[")?;

        for (index, record) in records.into_iter().enumerate() {
            match record.get("actor") {
                Some(actor) => {
                    // Add comma before non-first items
                    if processed_records > 0 {
                        output_file.write_all(b",")?;
                    }

                    // Write actor directly to file
                    let actor_json = serde_json::to_string(actor)?;
                    output_file.write_all(actor_json.as_bytes())?;

                    processed_records += 1;
                }
                None => {
                    println!("No actor found in record at index {}", index);
                    error_records += 1;
                }
            }
        }

        // Close JSON array
        output_file.write_all(b"]")?;
        output_file.flush()?;

        let duration = start_time.elapsed();

        // Debug: if no records processed, print more details
        if processed_records == 0 {
            println!(
                "No actors found. First 500 characters of file contents:\n{}",
                &file_contents[..std::cmp::min(500, file_contents.len())]
            );
        }

        Ok(ProcessingStats {
            total_records: processed_records + error_records,
            processed_records,
            error_records,
            processing_time: duration,
            output_file: output_path,
        })
    } else {
        // If not a JSON array, try parsing as JSON stream
        let stream =
            serde_json::Deserializer::from_str(&file_contents).into_iter::<serde_json::Value>();

        let start_time = Instant::now();
        let mut processed_records = 0;
        let mut error_records = 0;
        let output_path = PathBuf::from(config.json_dir.to_owned() + "actors.json");
        let mut output_file = File::create(&output_path)?;

        // Write opening bracket for JSON array
        output_file.write_all(b"[")?;

        for (index, record_result) in stream.enumerate() {
            match record_result {
                Ok(value) => {
                    // Extract actor from JSON value
                    if let Some(actor_value) = value.get("actor") {
                        // Add comma before non-first items
                        if processed_records > 0 {
                            output_file.write_all(b",")?;
                        }

                        // Write actor directly to file
                        let actor_json = serde_json::to_string(actor_value)?;
                        output_file.write_all(actor_json.as_bytes())?;

                        processed_records += 1;
                    } else {
                        println!("No actor found in record at index {}", index);
                        error_records += 1;
                    }
                }
                Err(e) => {
                    error_records += 1;
                    error!("Error parsing record at index {}: {}", index, e);
                    println!("Parsing error details: {:?}", e);
                }
            }
        }

        // Close JSON array
        output_file.write_all(b"]")?;
        output_file.flush()?;

        let duration = start_time.elapsed();

        // Debug: if no records processed, print more details
        if processed_records == 0 {
            println!(
                "No actors found. First 500 characters of file contents:\n{}",
                &file_contents[..std::cmp::min(500, file_contents.len())]
            );
        }

        Ok(ProcessingStats {
            total_records: processed_records + error_records,
            processed_records,
            error_records,
            processing_time: duration,
            output_file: output_path,
        })
    }
}
