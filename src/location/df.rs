use crate::location::location::{Hotspot, SubRegion};
use polars::prelude::{DataFrame, NamedFrom, Series};

pub fn sub_region_to_df(sub_regions: &[SubRegion]) -> DataFrame {
    let sub_region = sub_regions
        .iter()
        .map(|s| s.sub_region.to_owned())
        .collect::<Vec<_>>();
    let sub_region_code = sub_regions
        .iter()
        .map(|s| s.sub_region_code.to_owned())
        .collect::<Vec<_>>();
    let region = sub_regions.iter().map(|r| r.region()).collect::<Vec<_>>();
    let region_code = sub_regions
        .iter()
        .map(|r| r.region_code())
        .collect::<Vec<_>>();
    let country = sub_regions.iter().map(|r| r.country()).collect::<Vec<_>>();
    let country_code = sub_regions
        .iter()
        .map(|r| r.country_code())
        .collect::<Vec<_>>();
    match DataFrame::new(vec![
        Series::new("country", country),
        Series::new("country_code", country_code),
        Series::new("region", region),
        Series::new("region_code", region_code),
        Series::new("sub_region", sub_region),
        Series::new("sub_region_code", sub_region_code),
    ]) {
        Ok(df) => df,
        Err(e) => panic!("{:?}", e),
    }
}

pub fn hotspot_to_df(hotspots: &[Hotspot]) -> DataFrame {
    let hotspot = hotspots
        .iter()
        .map(|h| h.hotspot.to_owned())
        .collect::<Vec<_>>();
    let hotspot_code = hotspots
        .iter()
        .map(|h| h.hotspot_code.to_owned())
        .collect::<Vec<_>>();
    let sub_region = hotspots.iter().map(|h| h.sub_region()).collect::<Vec<_>>();
    let sub_region_code = hotspots
        .iter()
        .map(|h| h.sub_region_code())
        .collect::<Vec<_>>();
    let region = hotspots.iter().map(|h| h.region()).collect::<Vec<_>>();
    let region_code = hotspots.iter().map(|h| h.region_code()).collect::<Vec<_>>();
    let country = hotspots.iter().map(|h| h.country()).collect::<Vec<_>>();
    let country_code = hotspots
        .iter()
        .map(|h| h.country_code())
        .collect::<Vec<_>>();
    match DataFrame::new(vec![
        Series::new("country", country),
        Series::new("country_code", country_code),
        Series::new("region", region),
        Series::new("region_code", region_code),
        Series::new("sub_region", sub_region),
        Series::new("sub_region_code", sub_region_code),
        Series::new("hotspot", hotspot),
        Series::new("hotspot_code", hotspot_code),
    ]) {
        Ok(df) => df,
        Err(e) => panic!("{:?}", e),
    }
}
