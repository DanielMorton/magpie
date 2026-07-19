use crate::error::{AppError, Result};

#[derive(Debug, Clone)]
pub struct LocationRow {
    pub country: String,
    pub region: String,
    pub sub_region: String,
    pub hotspot: Option<String>,
}

impl LocationRow {
    pub fn from_csv_record(record: &csv::StringRecord, has_hotspot: bool) -> Result<Self> {
        let mut it = record.iter();
        let country = it.next().ok_or(AppError::Parse("Missing country".into()))?.to_string();
        let region = it.next().ok_or(AppError::Parse("Missing region".into()))?.to_string();
        let sub_region = it.next().ok_or(AppError::Parse("Missing sub_region".into()))?.to_string();

        if has_hotspot {
            let hotspot = it.next().ok_or(AppError::Parse("Missing hotspot".into()))?.to_string();
            Ok(Self::new_hotspot(country, region, sub_region, hotspot))
        } else {
            Ok(Self::new_location(country, region, sub_region))
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