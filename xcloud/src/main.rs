mod database;
mod middleware;
mod response;
mod server;
mod utils;

use database::Database;
use server::Server;
use sqlx::Error as SqlxError;
use std::io::Error as IoError;
use thiserror::Error;

/// Custom error type for the application.
#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Sqlx(#[from] SqlxError),

    #[error("IO error: {0}")]
    Io(#[from] IoError),
}

/// Main function for the application.
#[actix_web::main]
async fn main() -> Result<(), AppError> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("debug"));

    log::info!("Starting database...");
    log::info!("Starting server...");
    Server::new(Database::new().await?, "0.0.0.0:8080")
        .run()
        .await
        .map_err(AppError::from)?;
    log::info!("Database closed.");
    log::info!("Server closed.");
    Ok(())
}
