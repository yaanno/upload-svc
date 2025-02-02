use std::path::{Path, PathBuf};
use std::fs::File;
use actix_multipart::Multipart;
use futures::StreamExt;
use zip::ZipArchive;
use std::io::{BufReader, Write};

use crate::config::AppConfig;

pub async fn save_multipart_file(mut payload: Multipart, mut file: File) -> Result<(), Box<dyn std::error::Error>> {
    while let Some(item) = payload.next().await {
        let mut field = item?;
        while let Some(chunk) = field.next().await {
            let data = chunk?;
            file.write_all(&data)?;
        }
    }
    Ok(())
}

pub async fn validate_and_uncompress_zip(
    config: &AppConfig,
    file_path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
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
            let tmp = PathBuf::from(config.json_dir.to_owned());
            let mut outfile = File::create(tmp.join(&outpath))?;
            std::io::copy(&mut file, &mut outfile)?;
        }
    }

    Ok(())
}