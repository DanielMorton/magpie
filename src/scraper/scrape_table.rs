use crate::scraper::selectors::Selectors;
use crate::scraper::{CHECKLISTS, COMMON_NAME, PERCENT, SCIENTIFIC_NAME};
use polars::functions::concat_df_diagonal;
use polars::prelude::{DataFrame, NamedFrom, Series};
use scraper::ElementRef;
use std::str::FromStr;
use std::sync::Arc;

/**
Extracts common name for species in row.
*/
fn get_common_name(species: &Option<ElementRef>) -> String {
    species
        .and_then(|s| s.text().next())
        .unwrap_or("")
        .trim()
        .to_owned()
}

/**
Extracts scientific name for species in row.
*/
fn get_scientific_name(species: &Option<ElementRef>, selectors: &Arc<Selectors>) -> String {
    species
        .and_then(|s| {
            s.select(&selectors.sci_name)
                .next()
                .and_then(|s| s.text().next())
        })
        .unwrap_or("")
        .trim()
        .to_owned()
}

/**
Extracts common name and scientific name for species in row.
*/
fn get_species(row: &ElementRef, selectors: &Arc<Selectors>) -> (String, String) {
    let species = row
        .select(&selectors.species)
        .next()
        .and_then(|s| s.select(&selectors.a).next());
    (
        get_common_name(&species),
        get_scientific_name(&species, selectors),
    )
}

/**
Extracts the frequency of sightings as a percentage for species in row. Returns zero if
no percentage provided.
*/
fn get_percent(row: &ElementRef, selectors: &Arc<Selectors>) -> f32 {
    row.select(&selectors.percent)
        .next()
        .and_then(|p| p.value().attr("title"))
        .and_then(|p| p.split('%').next())
        .map(|p| match f32::from_str(p) {
            Ok(f) => f,
            Err(e) => panic!("No percentage for row {:?} \n {}", row, e),
        })
        .unwrap_or(0.0)
}

/**
Extracts species data from the table of all target species for a given location.
Returns common name (if present), scientific name (if present), and the frequency of sightings as
a floating point number. In rare cases, percentage can be greater than 100.
The three features and the number of checklists for the location in the relevant time interval are
returned in the form of a DataFrame.
*/
pub(super) fn scrape_table(
    selectors: &Arc<Selectors>,
    table: ElementRef,
    checklists: i32,
) -> DataFrame {
    let df_row = table
        .select(&selectors.rows)
        .map(|row| {
            let (common_name, scientific_name) = get_species(&row, selectors);
            let common_name_df = Series::new(COMMON_NAME, vec![common_name]);
            let scientific_name_df = Series::new(SCIENTIFIC_NAME, vec![scientific_name]);
            let percent = Series::new(PERCENT, vec![get_percent(&row, selectors)]);
            match DataFrame::new(vec![common_name_df, scientific_name_df, percent]) {
                Ok(df) => df,
                Err(e) => panic!("{}", e),
            }
        })
        .collect::<Vec<_>>();
    let checklist_column = Series::new(CHECKLISTS, vec![checklists; df_row.len()]);
    let mut df = match concat_df_diagonal(&df_row) {
        Ok(df) => df,
        Err(e) => panic!("{}", e),
    };
    match df.with_column(checklist_column) {
        Ok(_) => (),
        Err(e) => panic!("{}", e),
    }
    df
}
