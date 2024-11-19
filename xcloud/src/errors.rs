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
