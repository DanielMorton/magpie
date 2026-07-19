use std::num::ParseIntError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("CSV error: {0}")]
    Csv(#[from] csv::Error),

    #[error("Parse error: {0}")]
    Parse(String),

    #[error("Invalid month: {0}. Must be 1-12")]
    InvalidMonth(u8),

    #[error("Invalid time range: {0}. Expected format: '<start>-<end>'")]
    InvalidTimeRange(String),

    #[error("Location row must have 3 or 4 elements, got {0}")]
    InvalidElementCount(usize),

    #[error("No login token found in response")]
    MissingLoginToken,

    #[error("Max retries exceeded for URL: {0}")]
    MaxRetries(String),

    #[error("Invalid list type for location level")]
    InvalidListType,

    #[error("No output file specified")]
    MissingOutputFile,

    #[error("No location data provided")]
    MissingLocation,

    #[error("{0} scrapes failed. See failures.csv for details")]
    ScrapingFailures(usize),
}

impl From<ParseIntError> for AppError {
    fn from(_: ParseIntError) -> Self {
        AppError::Parse("Invalid integer".into())
    }
}

impl From<&str> for AppError {
    fn from(s: &str) -> Self {
        AppError::Parse(s.to_owned())
    }
}

pub type Result<T> = std::result::Result<T, AppError>;
