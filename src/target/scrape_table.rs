use crate::target::selectors::Selectors;
use crate::target::{CHECKLISTS, COMMON_NAME, PERCENT, SCIENTIFIC_NAME};
use polars::functions::concat_df_diagonal;
use polars::prelude::*;
use scraper::ElementRef;

/// Extracts common name for species in row.
fn get_common_name(species: &Option<ElementRef>) -> String {
    species
        .and_then(|s| s.text().next())
        .map(|s| s.trim().to_owned())
        .unwrap_or_default()
}

/// Extracts scientific name for species in row.
fn get_scientific_name(species: &Option<ElementRef>) -> String {
    species
        .and_then(|s| s.select(Selectors::sci_name()).next())
        .and_then(|s| s.text().next())
        .map(|s| s.trim().to_owned())
        .unwrap_or_default()
}

/// Extracts common name and scientific name for species in row.
fn get_species(row: &ElementRef) -> (String, String) {
    let species = row
        .select(Selectors::species())
        .next()
        .and_then(|s| s.select(Selectors::a()).next());
    (get_common_name(&species), get_scientific_name(&species))
}

/// Extracts the frequency of sightings as a percentage for species in row.
/// Returns zero if no percentage provided.
fn get_percent(row: &ElementRef) -> f32 {
    row.select(Selectors::percent())
        .next()
        .and_then(|p| p.value().attr("title"))
        .and_then(|p| p.split('%').next())
        .and_then(|p| p.parse::<f32>().ok())
        .unwrap_or(0.0)
}

/// Extracts species data from the table of all target species for a given location.
/// Returns common name (if present), scientific name (if present), and the frequency of sightings as
/// a floating point number. In rare cases, percentage can be greater than 100.
/// The three features and the number of checklists for the location in the relevant time interval are
/// returned in the form of a DataFrame.
pub(super) fn scrape_table(table: ElementRef, checklists: i32) -> Result<DataFrame, PolarsError> {
    let df_rows: Result<Vec<DataFrame>, PolarsError> = table
        .select(Selectors::rows())
        .map(|row| {
            let (common_name, scientific_name) = get_species(&row);
            let percent = get_percent(&row);
            df!(
                COMMON_NAME => [common_name],
                SCIENTIFIC_NAME => [scientific_name],
                PERCENT => [percent]
            )
        })
        .collect();

    let mut df = concat_df_diagonal(&df_rows?)?;
    df.with_column(Series::new(CHECKLISTS, vec![checklists; df.height()]))?;
    Ok(df)
}
