use crate::target::row::LocationRow;
use crate::target::{
    COMMON_NAME, COUNTRY, END_MONTH, HOTSPOT, PERCENT, REGION, SCIENTIFIC_NAME, START_MONTH,
    SUB_REGION,
};
use polars::prelude::{Column, DataFrame, NamedFrom, PolarsError, Series};

/// Adds columns that are constant for each scraped page. These columns are the location information:
/// sub-region, region, country, hotspot (if applicable), and the start and end months.
///
/// All columns are built up front and appended in a single `hstack_mut` call rather than one
/// `with_column` call per column, since each `with_column` re-validates the whole frame.
pub(super) fn add_columns(
    df: &mut DataFrame,
    row: &LocationRow,
    time: &[(String, u8)],
) -> Result<(), PolarsError> {
    let size = df.height();

    let mut new_columns: Vec<Column> = vec![
        Series::new(SUB_REGION.into(), vec![row.sub_region(); size]).into(),
        Series::new(REGION.into(), vec![row.region(); size]).into(),
        Series::new(COUNTRY.into(), vec![row.country(); size]).into(),
        Series::new(START_MONTH.into(), vec![time[0].1 as u32; size]).into(),
        Series::new(END_MONTH.into(), vec![time[1].1 as u32; size]).into(),
    ];

    if let Some(hotspot) = row.hotspot() {
        new_columns.push(Series::new(HOTSPOT.into(), vec![hotspot; size]).into());
    }

    df.hstack_mut(&new_columns)?;
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
