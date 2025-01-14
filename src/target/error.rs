use crate::target::scrape_params::LocationLevel;
use std::num::ParseIntError;
use polars::error::ErrString;
use polars::prelude::PolarsError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Invalid List Type for Location Level: {0}.")]
    InvalidListType(LocationLevel),

    #[error("Beginning Month must be between 1 and 12. {0} provided")]
    InvalidBeginMonth(String),

    #[error("Date Range not one of life, ytd, current_month, or date.")]
    InvalidDateRange,

    #[error("End Month must be between 1 and 12. {0} provided")]
    InvalidEndMonth(String),

    #[error(" Month must be between 1 and 12. {0} provided")]
    InvalidMonth(u8),

    #[error("Month must be between 1 and 12.")]
    InvalidNumberError,

    #[error("Time option not one of year, all, month, or range.")]
    InvalidTimeOption,

    #[error("Time range not in format \"<begin>-<end>\". {0} provided")]
    InvalidTimeRange(String),

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

#[derive(Error, Debug)]
pub enum LocationError {
    #[error("Location Row must have 3 or four elements, {0} provided")]
    InvalidElementCount(usize)
}

impl From<LocationError> for PolarsError {
    fn from(err: LocationError) -> Self {
        PolarsError::ComputeError(ErrString::from(format!("Location error: {:?}", err)))
    }
}
