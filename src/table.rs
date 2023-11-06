use crate::row::LocationRow;
use polars::frame::DataFrame;
use polars::prelude::NamedFrom;
use polars::series::Series;

pub(crate) fn add_columns(df: &mut DataFrame, row: &LocationRow, time: &Vec<(String, u8)>) {
    let size = df.shape().0;
    df.with_column(Series::new("sub_region", vec![row.sub_region(); size]))
        .unwrap();
    df.with_column(Series::new("region", vec![row.region(); size]))
        .unwrap();
    df.with_column(Series::new("country", vec![row.country(); size]))
        .unwrap();
    row.hotspot().iter().for_each(|hotspot| {
        df.with_column(Series::new("hotspot", vec![hotspot.clone(); size]))
            .unwrap();
    });
    df.with_column(Series::new("start month", vec![time[0].1 as u32; size]))
        .unwrap();
    df.with_column(Series::new("end month", vec![time[1].1 as u32; size]))
        .unwrap();
}

pub(crate) fn empty_table() -> DataFrame {
    let common_name = Series::new("common name", Vec::<String>::new());
    let scietific_name = Series::new("scientific name", Vec::<String>::new());
    let percent = Series::new("percent", Vec::<f32>::new());
    return DataFrame::new(vec![common_name, scietific_name, percent]).unwrap();
}
