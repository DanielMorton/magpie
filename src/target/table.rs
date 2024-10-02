use crate::target::row::LocationRow;
use crate::target::{
    COMMON_NAME, COUNTRY, END_MONTH, HOTSPOT, PERCENT, REGION, SCIENTIFIC_NAME, START_MONTH,
    SUB_REGION,
};
use polars::prelude::{DataFrame, NamedFrom, PolarsError, Series};

/// Adds columns that are constant for each scraped page. These columns are the location information:
/// sub-region, region, country, hotspot (if applicable), and the start and end months.
pub(super) fn add_columns(
    df: &mut DataFrame,
    row: &LocationRow,
    time: &[(String, u8)],
) -> Result<(), PolarsError> {
    let size = df.height();
    let constant_columns = [
        (SUB_REGION, row.sub_region()),
        (REGION, row.region()),
        (COUNTRY, row.country()),
    ];

    for (name, value) in constant_columns {
        df.with_column(Series::new(name, vec![value; size]))?;
    }

    if let Some(hotspot) = row.hotspot() {
        df.with_column(Series::new(HOTSPOT, vec![hotspot; size]))?;
    }

    df.with_column(Series::new(START_MONTH, vec![time[0].1 as u32; size]))?;
    df.with_column(Series::new(END_MONTH, vec![time[1].1 as u32; size]))?;

    Ok(())
}

/// In cases where there is no data to return, returns an empty table.
pub(super) fn empty_table() -> Result<DataFrame, PolarsError> {
    DataFrame::new(vec![
        Series::new(COMMON_NAME, Vec::<String>::new()),
        Series::new(SCIENTIFIC_NAME, Vec::<String>::new()),
        Series::new(PERCENT, Vec::<f32>::new()),
    ])
}