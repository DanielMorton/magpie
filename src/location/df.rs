use crate::location::country::{Country, Region, SubRegion};
use polars::prelude::{DataFrame, DataFrameJoinOps, NamedFrom, Series};

pub fn country_to_df(countries: &[Country]) -> DataFrame {
    let country = countries
        .iter()
        .map(|c| c.country.to_owned())
        .collect::<Vec<_>>();
    let country_code = countries
        .iter()
        .map(|c| c.country_code.to_owned())
        .collect::<Vec<_>>();
    match DataFrame::new(vec![
        Series::new("country", country),
        Series::new("country_code", country_code),
    ]) {
        Ok(df) => df,
        Err(e) => panic!("{:?}", e),
    }
}

pub fn region_to_df(regions: &[Region]) -> DataFrame {
    let region = regions
        .iter()
        .map(|r| r.region.to_owned())
        .collect::<Vec<_>>();
    let region_code = regions
        .iter()
        .map(|r| r.region_code.to_owned())
        .collect::<Vec<_>>();
    let country = regions.iter().map(|r| r.country()).collect::<Vec<_>>();
    let country_code = regions.iter().map(|r| r.country_code()).collect::<Vec<_>>();
    match DataFrame::new(vec![
        Series::new("country", country),
        Series::new("country_code", country_code),
        Series::new("region", region),
        Series::new("region_code", region_code),
    ]) {
        Ok(df) => df,
        Err(e) => panic!("{:?}", e),
    }
}

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

pub fn filter_join_df(
    left: &DataFrame,
    right: &DataFrame,
    join_cols: &[&str],
    filter_col: &str,
) -> DataFrame {
    match left.left_join(right, join_cols, join_cols).and_then(|df| {
        df.column(filter_col)
            .and_then(|s| Ok(s.is_null()))
            .and_then(|mask| df.filter(&mask))
            .and_then(|df| df.select(join_cols))
    }) {
        Ok(df) => df,
        Err(e) => panic!("{:?}", e),
    }
}
