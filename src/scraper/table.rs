use crate::scraper::row::LocationRow;
use crate::scraper::{
    COMMON_NAME, COUNTRY, END_MONTH, HOTSPOT, PERCENT, REGION, SCIENTIFIC_NAME, START_MONTH,
    SUB_REGION,
};
use polars::frame::DataFrame;
use polars::prelude::NamedFrom;
use polars::series::Series;

pub(super) fn add_columns(df: &mut DataFrame, row: &LocationRow, time: &[(String, u8)]) {
    let size = df.shape().0;
    match df.with_column(Series::new(SUB_REGION, vec![row.sub_region(); size])) {
        Ok(_) => (),
        Err(e) => panic!("{}", e)
    };
    match df.with_column(Series::new(REGION, vec![row.region(); size])){
        Ok(_) => (),
        Err(e) => panic!("{}", e)
    };
    match df.with_column(Series::new(COUNTRY, vec![row.country(); size])) {
        Ok(_) => (),
        Err(e) => panic!("{}", e)
    };
    row.hotspot().iter().for_each(|&hotspot| {
        match df.with_column(Series::new(HOTSPOT, vec![hotspot; size])) {
            Ok(_) => (),
            Err(e) => panic!("{}", e)
        }
    });
    match df.with_column(Series::new(START_MONTH, vec![time[0].1 as u32; size])) {
        Ok(_) => (),
        Err(e) => panic!("{}", e)
    };
    match df.with_column(Series::new(END_MONTH, vec![time[1].1 as u32; size])) {
        Ok(_) => (),
        Err(e) => panic!("{}", e)
    };
}

pub(super) fn empty_table() -> DataFrame {
    let common_name = Series::new(COMMON_NAME, Vec::<String>::new());
    let scietific_name = Series::new(SCIENTIFIC_NAME, Vec::<String>::new());
    let percent = Series::new(PERCENT, Vec::<f32>::new());
    match DataFrame::new(vec![common_name, scietific_name, percent]) {
        Ok(df) => df,
        Err(e) => panic!("{}", e)
    }
}
