use crate::target::row::LocationRow;
use polars::prelude::{DataFrame, Series};
use crate::error::Result;

pub fn add_columns(df: &mut DataFrame, row: &LocationRow, time: &[(String, u8)]) -> Result<()> {
    let size = df.height();
    df.with_column(Series::new("sub_region".into(), vec![row.sub_region.as_str(); size]))?;
    df.with_column(Series::new("region".into(), vec![row.region.as_str(); size]))?;
    df.with_column(Series::new("country".into(), vec![row.country.as_str(); size]))?;

    if let Some(hotspot) = &row.hotspot {
        df.with_column(Series::new("hotspot".into(), vec![hotspot.as_str(); size]))?;
    }

    df.with_column(Series::new("start month".into(), vec![time[0].1 as u32; size]))?;
    df.with_column(Series::new("end month".into(), vec![time[1].1 as u32; size]))?;
    Ok(())
}

pub fn empty_table() -> Result<DataFrame> {
    DataFrame::new(vec![
        Series::new("common name".into(), Vec::<String>::new()),
        Series::new("scientific name".into(), Vec::<String>::new()),
        Series::new("percent".into(), Vec::<f32>::new()),
    ]).map_err(Into::into)
}