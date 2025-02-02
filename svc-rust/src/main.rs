mod config;
mod handlers;
mod types;
mod utils;

use actix_web::{web, App, HttpServer};
use env_logger::{self, Env};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = config::AppConfig::default();
    config.create_dirs().unwrap();

    // Configure logging
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();

    let config_clone = config.clone(); // Create a clone for the bind method

    HttpServer::new(move || {
        App::new()
            // .wrap(TracingLogger::default())
            .app_data(web::Data::new(config.clone()))
            .service(handlers::upload_zip)
            .service(handlers::upload_large_zip)
    })
    .bind((config_clone.server_host.as_str(), config_clone.server_port))?
    .run()
    .await
}
