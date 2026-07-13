use crate::location::loc::{Hotspot, SubRegion};
use polars::prelude::{Column, DataFrame, NamedFrom, Series};
use std::error::Error;

pub fn sub_region_to_df(sub_regions: &[SubRegion]) -> Result<DataFrame, Box<dyn Error>> {
    let columns: Vec<Column> = vec![
        Series::new(
            "country".into(),
            sub_regions.iter().map(|r| r.country()).collect::<Vec<_>>(),
        )
            .into(),
        Series::new(
            "country_code".into(),
            sub_regions
                .iter()
                .map(|r| r.country_code())
                .collect::<Vec<_>>(),
        )
            .into(),
        Series::new(
            "region".into(),
            sub_regions.iter().map(|r| r.region()).collect::<Vec<_>>(),
        )
            .into(),
        Series::new(
            "region_code".into(),
            sub_regions
                .iter()
                .map(|r| r.region_code())
                .collect::<Vec<_>>(),
        )
            .into(),
        Series::new(
            "sub_region".into(),
            sub_regions
                .iter()
                .map(|s| s.sub_region())
                .collect::<Vec<_>>(),
        )
            .into(),
        Series::new(
            "sub_region_code".into(),
            sub_regions
                .iter()
                .map(|s| s.sub_region_code())
                .collect::<Vec<_>>(),
        )
            .into(),
    ];

    DataFrame::new(sub_regions.len(), columns).map_err(|e| e.into())
}

pub fn hotspot_to_df(hotspots: &[Hotspot]) -> Result<DataFrame, Box<dyn Error>> {
    let columns: Vec<Column> = vec![
        Series::new(
            "country".into(),
            hotspots.iter().map(|h| h.country()).collect::<Vec<_>>(),
        )
            .into(),
        Series::new(
            "country_code".into(),
            hotspots
                .iter()
                .map(|h| h.country_code())
                .collect::<Vec<_>>(),
        )
            .into(),
        Series::new(
            "region".into(),
            hotspots.iter().map(|h| h.region()).collect::<Vec<_>>(),
        )
            .into(),
        Series::new(
            "region_code".into(),
            hotspots.iter().map(|h| h.region_code()).collect::<Vec<_>>(),
        )
            .into(),
        Series::new(
            "sub_region".into(),
            hotspots.iter().map(|h| h.sub_region()).collect::<Vec<_>>(),
        )
            .into(),
        Series::new(
            "sub_region_code".into(),
            hotspots
                .iter()
                .map(|h| h.sub_region_code())
                .collect::<Vec<_>>(),
        )
            .into(),
        Series::new(
            "hotspot".into(),
            hotspots.iter().map(|h| h.hotspot()).collect::<Vec<_>>(),
        )
            .into(),
        Series::new(
            "hotspot_code".into(),
            hotspots
                .iter()
                .map(|h| h.hotspot_code())
                .collect::<Vec<_>>(),
        )
            .into(),
    ];

    DataFrame::new(hotspots.len(), columns).map_err(|e| e.into())
}