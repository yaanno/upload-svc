use std::env;

#[derive(Debug, Clone,)]
pub struct AppConfig {
    pub json_dir: String,
    pub large_json_dir: String,
    pub upload_dir: String,
    pub large_upload_dir: String,
    pub max_file_size_mb: usize,
    pub upload_file_name: String,
    pub server_host: String,
    pub server_port: u16,
}

impl AppConfig {
    pub fn new() -> Self {
        Self {
            json_dir: env::var("JSON_DIR").unwrap_or_else(|_| "./tmp/".to_string()),
            large_json_dir: env::var("LARGE_JSON_DIR").unwrap_or_else(|_| "./tmp-large/".to_string()),
            upload_dir: env::var("UPLOAD_DIR").unwrap_or_else(|_| "./uploads/".to_string()),
            large_upload_dir: env::var("LARGE_UPLOAD_DIR").unwrap_or_else(|_| "./uploads-large/".to_string()),
            max_file_size_mb: env::var("MAX_FILE_SIZE_MB")
                .map(|v| v.parse().unwrap_or(500))
                .unwrap_or(500),
            upload_file_name: env::var("UPLOAD_FILE_NAME").unwrap_or_else(|_| "upload.zip".to_string()),
            server_host: env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
            server_port: env::var("SERVER_PORT")
                .map(|v| v.parse().unwrap_or(8080))
                .unwrap_or(8080),
        }
    }

    pub fn create_dirs(&self) -> Result<(), std::io::Error> {
        std::fs::create_dir_all(&self.json_dir)?;
        std::fs::create_dir_all(&self.upload_dir)?;
        std::fs::create_dir_all(&self.large_json_dir)?;
        std::fs::create_dir_all(&self.large_upload_dir)?;
        Ok(())
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for AppConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "JSON_DIR: {}, LARGE_JSON_DIR: {}, UPLOAD_DIR: {}, LARGE_UPLOAD_DIR: {}, MAX_FILE_SIZE_MB: {}, UPLOAD_FILE_NAME: {}, SERVER_HOST: {}, SERVER_PORT: {}",
            self.json_dir, self.large_json_dir, self.upload_dir, self.large_upload_dir, self.max_file_size_mb, self.upload_file_name, self.server_host, self.server_port
        )
    }
}
