use actix_web::web;

use crate::config::AppConfig;
use crate::types::Actor;
use crate::utils::json_processing::{
    process_json_file, process_large_json_stream, ProcessingStats,
};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};

pub(crate) fn process_json_dir(
    config: web::Data<AppConfig>,
) -> Result<(), Box<dyn std::error::Error>> {
    //info!("Starting processing the json files");
    let nested_actors: Vec<Vec<Actor>> = std::fs::read_dir(Path::new(config.json_dir.as_str()))?
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().file_name().and_then(|s| s.to_str()) != Some("actors.json"))
        .filter(|entry| entry.path().extension().and_then(|s| s.to_str()) == Some("json"))
        .map(|entry| process_json_file(&entry.path()))
        .collect::<Result<Vec<_>, _>>()?;
    let actors: Vec<Actor> = nested_actors.into_iter().flatten().collect();

    let path = PathBuf::from(config.json_dir.to_owned() + config.upload_file_name.as_str());
    let actors_json = serde_json::to_string(&actors)?;
    let file = File::create(path)?;

    let mut writer = BufWriter::new(file);
    writer.write_all(actors_json.as_bytes())?;
    writer.flush()?;

    Ok(())
}

pub(crate) fn process_large_json_dir(
    config: web::Data<AppConfig>,
) -> Result<(), Box<dyn std::error::Error>> {
    //info!("Starting processing the json files");
    let nested_actors: Vec<ProcessingStats> =
        std::fs::read_dir(Path::new(config.json_dir.as_str()))?
            .filter_map(|entry| entry.ok())
            .filter(|entry| {
                entry.path().file_name().and_then(|s| s.to_str()) != Some("actors-stream.json")
            })
            .filter(|entry| entry.path().extension().and_then(|s| s.to_str()) == Some("json"))
            .map(|entry| process_large_json_stream(&config, &entry.path(), 1000000000))
            .collect::<Result<Vec<_>, _>>()?;
    println!("{:?}", nested_actors);

    Ok(())
}
