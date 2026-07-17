use crate::error::{AppError, Result};
use polars::series::SeriesIter;

#[derive(Debug, Clone)]
pub struct LocationRow {
    pub country: String,
    pub region: String,
    pub sub_region: String,
    pub hotspot: Option<String>,
}

impl LocationRow {
    pub fn from_iters(iters: &mut [SeriesIter]) -> Result<Self> {
        let next = |i: usize| -> Result<String> {
            iters[i].next()
                .map(|v| v.to_string().trim_matches('"').to_owned())
                .ok_or_else(|| AppError::Parse("Missing column value".into()))
        };

        match iters.len() {
            3 => Ok(Self::new_location(next(0)?, next(1)?, next(2)?)),
            4 => Ok(Self::new_hotspot(next(0)?, next(1)?, next(2)?, next(3)?)),
            n => Err(AppError::InvalidElementCount(n)),
        }
    }

    pub fn new_hotspot(country: String, region: String, sub_region: String, hotspot: String) -> Self {
        Self { country, region, sub_region, hotspot: Some(hotspot) }
    }

    pub fn new_location(country: String, region: String, sub_region: String) -> Self {
        Self { country, region, sub_region, hotspot: None }
    }
}