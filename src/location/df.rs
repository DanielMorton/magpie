use crate::location::loc::{Hotspot, SubRegion};
use polars::prelude::{DataFrame, NamedFrom, Series};
use std::error::Error;

pub fn sub_region_to_df(sub_regions: &[SubRegion]) -> Result<DataFrame, Box<dyn Error>> {
    DataFrame::new(vec![
        Series::new(
            "country",
            sub_regions.iter().map(|r| r.country()).collect::<Vec<_>>(),
        ),
        Series::new(
            "country_code",
            sub_regions
                .iter()
                .map(|r| r.country_code())
                .collect::<Vec<_>>(),
        ),
        Series::new(
            "region",
            sub_regions.iter().map(|r| r.region()).collect::<Vec<_>>(),
        ),
        Series::new(
            "region_code",
            sub_regions
                .iter()
                .map(|r| r.region_code())
                .collect::<Vec<_>>(),
        ),
        Series::new(
            "sub_region",
            sub_regions
                .iter()
                .map(|s| s.sub_region())
                .collect::<Vec<_>>(),
        ),
        Series::new(
            "sub_region_code",
            sub_regions
                .iter()
                .map(|s| s.sub_region_code())
                .collect::<Vec<_>>(),
        ),
    ])
    .map_err(|e| e.into())
}

pub fn hotspot_to_df(hotspots: &[Hotspot]) -> Result<DataFrame, Box<dyn Error>> {
    DataFrame::new(vec![
        Series::new(
            "country",
            hotspots.iter().map(|h| h.country()).collect::<Vec<_>>(),
        ),
        Series::new(
            "country_code",
            hotspots
                .iter()
                .map(|h| h.country_code())
                .collect::<Vec<_>>(),
        ),
        Series::new(
            "region",
            hotspots.iter().map(|h| h.region()).collect::<Vec<_>>(),
        ),
        Series::new(
            "region_code",
            hotspots.iter().map(|h| h.region_code()).collect::<Vec<_>>(),
        ),
        Series::new(
            "sub_region",
            hotspots.iter().map(|h| h.sub_region()).collect::<Vec<_>>(),
        ),
        Series::new(
            "sub_region_code",
            hotspots
                .iter()
                .map(|h| h.sub_region_code())
                .collect::<Vec<_>>(),
        ),
        Series::new(
            "hotspot",
            hotspots.iter().map(|h| h.hotspot()).collect::<Vec<_>>(),
        ),
        Series::new(
            "hotspot_code",
            hotspots
                .iter()
                .map(|h| h.hotspot_code())
                .collect::<Vec<_>>(),
        ),
    ])
    .map_err(|e| e.into())
}
