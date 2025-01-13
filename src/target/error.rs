use crate::target::scrape_params::LocationLevel;
use std::num::ParseIntError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Invalid List Type for Location Level: {0}.")]
    InvalidListType(LocationLevel),

    #[error("Date Range not one of life, ytd, current_month, or date.")]
    InvalidDateRange,

    #[error("Month must be between 1 and 12.")]
    InvalidNumberError,

    #[error("Time range not one of year, all, month, or range.")]
    InvalidTimeRange,

    #[error("No sub-region or hotspot list provided.")]
    InvalidLocationType,

    #[error("No output file provided.")]
    MissingOutputFile,
}

impl From<ParseIntError> for ParseError {
    fn from(_: ParseIntError) -> Self {
        ParseError::InvalidNumberError
    }
}
