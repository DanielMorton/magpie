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
        df.with_column(Series::new(name.into(), vec![value; size]).into())?;
    }

    if let Some(hotspot) = row.hotspot() {
        df.with_column(Series::new(HOTSPOT.into(), vec![hotspot; size]).into())?;
    }

    df.with_column(Series::new(START_MONTH.into(), vec![time[0].1 as u32; size]).into())?;
    df.with_column(Series::new(END_MONTH.into(), vec![time[1].1 as u32; size]).into())?;

    Ok(())
}

/// In cases where there is no data to return, returns an empty table.
pub(super) fn empty_table() -> Result<DataFrame, PolarsError> {
    let columns = vec![
        Series::new(COMMON_NAME.into(), Vec::<String>::new()).into(),
        Series::new(SCIENTIFIC_NAME.into(), Vec::<String>::new()).into(),
        Series::new(PERCENT.into(), Vec::<f32>::new()).into(),
    ];
    DataFrame::new(0, columns)
}