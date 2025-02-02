use std::{
    fs::File,
    io::{BufReader, Write},
    path::{Path, PathBuf},
};

use tracing::error;

use crate::{
    config,
    types::{Actor, Event},
};

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
) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let _reader = BufReader::new(file);

    // Read entire file contents to handle no-line-terminator case
    let file_contents = std::fs::read_to_string(file_path)?;

    // Try parsing as JSON array first
    if let Ok(records) = serde_json::from_str::<Vec<serde_json::Value>>(&file_contents) {
        let output_path = PathBuf::from(config.json_dir.to_owned() + "actors-stream.json");
        let mut output_file = File::create(&output_path)?;

        // Write opening bracket for JSON array
        output_file.write_all(b"[")?;

        for (index, record) in records.into_iter().enumerate() {
            match record.get("actor") {
                Some(actor) => {
                    // Add comma before non-first items

                    output_file.write_all(b",")?;

                    // Write actor directly to file
                    let actor_json = serde_json::to_string(actor)?;
                    output_file.write_all(actor_json.as_bytes())?;
                }
                None => {
                    println!("No actor found in record at index {}", index);
                }
            }
        }

        // Close JSON array
        output_file.write_all(b"]")?;
        output_file.flush()?;

        Ok(())
    } else {
        // If not a JSON array, try parsing as JSON stream
        let stream =
            serde_json::Deserializer::from_str(&file_contents).into_iter::<serde_json::Value>();
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
                        if index > 0 {
                            output_file.write_all(b",")?;
                        }

                        // Write actor directly to file
                        let actor_json = serde_json::to_string(actor_value)?;
                        output_file.write_all(actor_json.as_bytes())?;
                    } else {
                        println!("No actor found in record at index {}", index);
                    }
                }
                Err(e) => {
                    error!("Error parsing record at index {}: {}", index, e);
                    println!("Parsing error details: {:?}", e);
                }
            }
        }

        // Close JSON array
        output_file.write_all(b"]")?;
        output_file.flush()?;

        // Debug: if no records processed, print more details

        Ok(())
    }
}
