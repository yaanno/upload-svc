use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Debug, Serialize, Deserialize)]
struct Event {
    actor: Actor,
    event: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Actor {
    name: String,
    age: u32,
}


pub(crate) fn process_json_file(
    input_path: &str,
    output_path: &str,
) -> Result<Vec<Actor>, Box<dyn std::error::Error>> {
    let file = File::open(input_path)?;
    let reader = BufReader::new(file);

    // Use serde_json to parse the entire file as a JSON array
    let records: Vec<Event> = serde_json::from_reader(reader)?;
    let actors: Vec<Actor> = records
        .into_iter()
        .filter_map(|record| Some(record.actor))
        .collect();

    // Prepare output file path
    let output_path = PathBuf::from(output_path);

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


fn generate_large_test_file<P: AsRef<Path>>(
    path: P, 
    num_events: usize
) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);

    // Write JSON array start
    writer.write_all(b"[\n")?;

    for i in 0..num_events {
        let event = Event {
            actor: Actor {
                name: format!("Actor_{}", i),
                age: (i % 100) as u32,
            },
            event: if i % 2 == 0 { "login".to_string() } else { "logout".to_string() },
        };

        // Add comma for non-first items
        if i > 0 {
            writer.write_all(b",\n")?;
        }

        // Write event
        let event_json = serde_json::to_string(&event)?;
        writer.write_all(event_json.as_bytes())?;
    }

    // Close JSON array
    writer.write_all(b"\n]")?;
    writer.flush()?;

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Paths for input and output
    let input_path = "./large_events.json";
    let output_path = "./processed_actors.json";

    // Generate a large test file
    println!("Generating large test file...");
    generate_large_test_file(input_path, 10_000_000)?;

    // Process the large file
    println!("Processing large JSON stream...");
    let start = std::time::Instant::now();
    
    process_json_file(input_path, output_path)?;
    
    let duration = start.elapsed();
    println!("Processing completed in {:?}", duration);

    // Optional: Verify output
    let output_metadata = std::fs::metadata(output_path)?;
    println!("Output file size: {} bytes", output_metadata.len());

    Ok(())
}
