mod types;

use actix_web::{post, web, App, Error, HttpResponse, HttpServer};
use env_logger::{self, Env};
use futures::StreamExt;
use log::{error, info};

use serde_json::Deserializer;
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};
use std::time::Instant;
use tracing_actix_web::TracingLogger;
use types::{Actor, GithubActions};
use zip::ZipArchive;

const JSON_DIR: &str = "./tmp/";
const OUT_FILE: &str = "actors.json";
const UPLOADED_FILE: &str = "uploaded.zip";

fn process_json_file(file_path: &Path) -> Result<Vec<Actor>, Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    // Create a streaming deserializer
    let stream = Deserializer::from_reader(reader).into_iter::<GithubActions>();
    let mut actors = Vec::new();
    for record in stream {
        match record {
            Ok(record) => {
                actors.extend(record.iter().map(|item| item.actor.clone()));
                info!("Json file processed: {}", &file_path.display());
            }
            Err(e) => {
                error!("Error parsing record: {}", e);
            }
        }
    }

    Ok(actors)
}

fn process_json_dir() -> Result<(), Box<dyn std::error::Error>> {
    info!("Starting processing the json files");
    let nested_actors: Vec<Vec<Actor>> = std::fs::read_dir(Path::new(JSON_DIR))?
        .filter_map(|entry| entry.ok())
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

    info!("Starting to uncompress the ZIP file.");

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
            info!("outpath: {:?}", outpath);
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
            info!("Extracted file: {:?}", outpath);
        }
    }

    info!("Finished uncompressing the ZIP file.");
    Ok(())
}

#[post("/upload")]
async fn upload_zip(mut payload: web::Payload) -> Result<HttpResponse, Error> {
    let start_time = Instant::now();
    // Create a file to save the uploaded ZIP
    let file_path = PathBuf::from(JSON_DIR.to_owned() + UPLOADED_FILE);
    let file = File::create(&file_path)?;
    let mut writer = BufWriter::new(file);

    info!("Starting to receive the uploaded ZIP file.");

    // Stream the payload and write to the file in chunks
    while let Some(chunk) = payload.next().await {
        let data = chunk?;
        writer.write_all(&data)?;
    }

    // Ensure all data is written to the file
    writer.flush()?;
    info!("Finished writing the uploaded ZIP file to disk.");

    match validate_and_uncompress_zip(&file_path) {
        Ok(_) => {
            process_json_dir()?;
            info!(
                "Files processed. Output written to: {}",
                OUT_FILE.to_owned()
            );
            let end_time = Instant::now();
            let elapsed_time = end_time - start_time;
            info!("Elapsed time: {:?}", elapsed_time);
            Ok(HttpResponse::Ok().body("File uploaded and uncompressed successfully"))
        }
        Err(e) => {
            error!("File processing failed: {:?}", e);
            Ok(HttpResponse::BadRequest().body("File processing failed"))
        }
    }
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .wrap(TracingLogger::default())
            .service(upload_zip)
    })
    .workers(4)
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
