use std::{io, time::SystemTimeError};
use thiserror::Error;
pub type Result<T> = std::result::Result<T, EleaError>;

#[derive(Error, Debug)]
pub enum EleaError {
    #[error("Io Error: {0}")]
    Io(#[from] io::Error),

    #[error("Invalid data provided: {0}")]
    InvalidData(String),

    #[error("Operation timed out after {0}ms")]
    Timeout(u64),

    #[error("Time error: {0}")]
    Time(#[from] SystemTimeError),

    #[error("An unknown error occurred")]
    Unknown,
}
