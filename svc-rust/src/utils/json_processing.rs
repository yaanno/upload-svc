use std::{
    fs::File,
    io::{BufReader, Write},
    path::{Path, PathBuf},
};

use crate::{
    config,
    types::{Actor, Event},
};

pub(crate) fn process_json_file(
    config: &config::AppConfig,
    file_path: &Path,
) -> Result<Vec<Actor>, Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    // Use serde_json to parse the entire file as a JSON array
    let records: Vec<Event> = serde_json::from_reader(reader)?;
    let actors: Vec<Actor> = records
        .into_iter()
        .filter_map(|record| record.actor)
        .collect();

    // Prepare output file path
    let output_path = PathBuf::from(config.json_dir.to_owned() + "actors.json");

    // Open file for writing
    let mut output_file = File::create(&output_path)?;

    // Write JSON string to file
    let formatted_json = serde_json::to_string_pretty(&actors)
        .map_err(|e| format!("Failed to serialize actors: {}", e))?;

    // Write the formatted JSON string
    output_file
        .write_all(formatted_json.as_bytes())
        .map_err(|e| format!("Failed to write to file {}: {}", output_path.display(), e))?;

    output_file.flush()?;

    Ok(actors)
}

pub fn process_large_json_stream(
    config: &config::AppConfig,
    file_path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    // Create a streaming JSON deserializer
    let stream = serde_json::Deserializer::from_reader(reader).into_iter::<serde_json::Value>();

    // Prepare output file
    let output_path = PathBuf::from(config.json_dir.to_owned() + "actors-stream.json");
    let mut output_file = File::create(&output_path)?;

    // Write opening bracket for JSON array
    output_file.write_all(b"[")?;

    // Track whether we've written any actors to handle comma placement
    let mut first_actor = true;

    // Stream through the JSON records
    for (index, record_result) in stream.enumerate() {
        match record_result {
            Ok(value) => {
                // Extract actor from JSON value
                if let Some(actor_value) = value.get("actor") {
                    // Add comma before non-first items
                    if !first_actor {
                        output_file.write_all(b",")?;
                    } else {
                        first_actor = false;
                    }

                    // Write actor directly to file
                    let actor_json = serde_json::to_string(actor_value)?;
                    output_file.write_all(actor_json.as_bytes())?;
                } else {
                    tracing::debug!("No actor found in record at index {}", index);
                }
            }
            Err(e) => {
                tracing::error!("Error parsing record at index {}: {}", index, e);
            }
        }
    }

    // Close JSON array
    output_file.write_all(b"]")?;
    output_file.flush()?;

    Ok(())
}
