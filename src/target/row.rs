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
        let values: Result<Vec<String>> = (0..iters.len())
            .map(|i| {
                iters[i]
                    .next()
                    .map(|v| v.to_string().trim_matches('"').to_owned())
                    .ok_or_else(|| AppError::Parse("Missing column value".into()))
            })
            .collect();

        let vals = values?;
        match vals.len() {
            3 => Ok(Self::new_location(
                vals[0].clone(),
                vals[1].clone(),
                vals[2].clone(),
            )),
            4 => Ok(Self::new_hotspot(
                vals[0].clone(),
                vals[1].clone(),
                vals[2].clone(),
                vals[3].clone(),
            )),
            n => Err(AppError::InvalidElementCount(n)),
        }
    }

    pub fn new_hotspot(
        country: String,
        region: String,
        sub_region: String,
        hotspot: String,
    ) -> Self {
        Self {
            country,
            region,
            sub_region,
            hotspot: Some(hotspot),
        }
    }

    pub fn new_location(country: String, region: String, sub_region: String) -> Self {
        Self {
            country,
            region,
            sub_region,
            hotspot: None,
        }
    }
}
