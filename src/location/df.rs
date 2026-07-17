use crate::error::Result;
use crate::location::loc::{Hotspot, SubRegion};
use polars::prelude::{Column, DataFrame, NamedFrom, Series};

pub fn sub_regions_to_df(sub_regions: &[SubRegion]) -> Result<DataFrame> {
    let len = sub_regions.len();
    DataFrame::new(len, vec![
        Column::from(Series::new("country".into(), sub_regions.iter().map(|r| r.country()).collect::<Vec<_>>())),
        Column::from(Series::new("country_code".into(), sub_regions.iter().map(|r| r.country_code()).collect::<Vec<_>>())),
        Column::from(Series::new("region".into(), sub_regions.iter().map(|r| r.region()).collect::<Vec<_>>())),
        Column::from(Series::new("region_code".into(), sub_regions.iter().map(|r| r.region_code()).collect::<Vec<_>>())),
        Column::from(Series::new("sub_region".into(), sub_regions.iter().map(|s| s.name()).collect::<Vec<_>>())),
        Column::from(Series::new("sub_region_code".into(), sub_regions.iter().map(|s| s.code()).collect::<Vec<_>>())),
    ])
        .map_err(Into::into)
}

pub fn hotspots_to_df(hotspots: &[Hotspot]) -> Result<DataFrame> {
    let len = hotspots.len();
    DataFrame::new(len, vec![
        Column::from(Series::new("country".into(), hotspots.iter().map(|h| h.country()).collect::<Vec<_>>())),
        Column::from(Series::new("country_code".into(), hotspots.iter().map(|h| h.country_code()).collect::<Vec<_>>())),
        Column::from(Series::new("region".into(), hotspots.iter().map(|h| h.region()).collect::<Vec<_>>())),
        Column::from(Series::new("region_code".into(), hotspots.iter().map(|h| h.region_code()).collect::<Vec<_>>())),
        Column::from(Series::new("sub_region".into(), hotspots.iter().map(|h| h.sub_region()).collect::<Vec<_>>())),
        Column::from(Series::new("sub_region_code".into(), hotspots.iter().map(|h| h.sub_region_code()).collect::<Vec<_>>())),
        Column::from(Series::new("hotspot".into(), hotspots.iter().map(|h| h.name()).collect::<Vec<_>>())),
        Column::from(Series::new("hotspot_code".into(), hotspots.iter().map(|h| h.code()).collect::<Vec<_>>())),
    ])
        .map_err(Into::into)
}