use actix_web::web;

use crate::config::AppConfig;
use crate::types::Actor;
use crate::utils::json_processing::{
    process_json_file, process_large_json_stream,
};
use std::fmt::Debug;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};

// #[derive(Debug)]
struct ProcessingConfig {
    /// Filename to exclude from processing
    exclude_filename: &'static str,
    /// Output filename
    output_filename: &'static str,
    /// Processing strategy (closure that defines how to process files)
    processing_strategy: Box<dyn Fn(&AppConfig, &Path) -> Result<Vec<Actor>, Box<dyn std::error::Error>>>,
    /// Large processing strategy (for stream processing)
    large_processing_strategy: Box<dyn Fn(&AppConfig, &Path) -> Result<(), Box<dyn std::error::Error>>>,
}

impl Debug for ProcessingConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ProcessingConfig")
            .field("exclude_filename", &self.exclude_filename)
            .field("output_filename", &self.output_filename)
            .finish()
    }
}

impl Default for ProcessingConfig {
    fn default() -> Self {
        Self {
            exclude_filename: "actors.json",
            output_filename: "actors.json",
            processing_strategy: Box::new(|config, path| process_json_file(&config, path)),
            large_processing_strategy: Box::new(|config, path| 
                process_large_json_stream(config, path)
            ),
        }
    }
}

fn process_directory(
    config: &AppConfig, 
    processing_config: &ProcessingConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    let nested_actors: Vec<Vec<Actor>> = std::fs::read_dir(Path::new(config.json_dir.as_str()))?
        .filter_map(|entry| entry.ok())
        .filter(|entry| 
            entry.path().file_name().and_then(|s| s.to_str()) 
            != Some(processing_config.exclude_filename)
        )
        .filter(|entry| 
            entry.path().extension().and_then(|s| s.to_str()) 
            == Some("json")
        )
        .map(|entry| (processing_config.processing_strategy)(&config, &entry.path()))
        .collect::<Result<Vec<_>, _>>()?;

    let actors: Vec<Actor> = nested_actors.into_iter().flatten().collect();

    let path = PathBuf::from(config.json_dir.to_owned() + processing_config.output_filename);
    let actors_json = serde_json::to_string(&actors)?;
    let file = File::create(path)?;

    let mut writer = BufWriter::new(file);
    writer.write_all(actors_json.as_bytes())?;
    writer.flush()?;

    Ok(())
}

fn process_large_directory(
    config: &AppConfig, 
    processing_config: &ProcessingConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    let nested_actors: Vec<()> = std::fs::read_dir(Path::new(config.large_json_dir.as_str()))?
        .filter_map(|entry| entry.ok())
        .filter(|entry| 
            entry.path().file_name().and_then(|s| s.to_str()) 
            != Some(processing_config.exclude_filename)
        )
        .filter(|entry| 
            entry.path().extension().and_then(|s| s.to_str()) 
            == Some("json")
        )
        .map(|entry| 
            (processing_config.large_processing_strategy)(config, &entry.path())
        )
        .collect::<Result<Vec<_>, _>>()?;

    println!("{:?}", nested_actors.len());

    Ok(())
}

pub(crate) fn process_json_dir(
    config: web::Data<AppConfig>,
) -> Result<(), Box<dyn std::error::Error>> {
    let processing_config = ProcessingConfig {
        exclude_filename: "actors.json",
        output_filename: "actors.json",
        processing_strategy: Box::new(|config, path| process_json_file(config, path)),
        large_processing_strategy: Box::new(|config, path| 
            process_large_json_stream(config, path)
        ),  
    };
    process_directory(&config, &processing_config)
}

pub(crate) fn process_large_json_dir(
    config: web::Data<AppConfig>,
) -> Result<(), Box<dyn std::error::Error>> {
    let processing_config = ProcessingConfig {
        exclude_filename: "actors-stream.json",
        output_filename: "actors-stream.json",
        processing_strategy: Box::new(|config, path| process_json_file(config, path)),
        large_processing_strategy: Box::new(|config, path| 
            process_large_json_stream(config, path)
        ),
    };
    process_large_directory(&config, &processing_config)
}
