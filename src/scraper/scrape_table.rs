use crate::scraper::row::SpeciesRow;
use crate::scraper::selectors::Selectors;
use crate::scraper::{CHECKLISTS, COMMON_NAME, PERCENT, SCIENTIFIC_NAME};
use polars::prelude::{DataFrame, NamedFrom, Series};
use scraper::ElementRef;
use std::str::FromStr;
use std::sync::Arc;

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
        .select(selectors.rows())
        .map(|row| {
            let species = row
                .select(selectors.species())
                .next()
                .and_then(|s| s.select(selectors.a()).next());
            // Extracts common name of species. Returns an empty string if not present.
            let common_name = species.and_then(|s| s.text().next()).unwrap_or("").trim();

            // Extracts scientific name of species. Returns an empty string if not present.
            let scientific_name = species
                .and_then(|s| {
                    s.select(selectors.sci_name())
                        .next()
                        .and_then(|s| s.text().next())
                })
                .unwrap_or("")
                .trim();

            // Extracts frequency of sightings as a percentage. Returns zero if not present.
            let percent = row
                .select(selectors.percent())
                .next()
                .and_then(|p| p.value().attr("title"))
                .and_then(|p| p.split('%').next())
                .map(|p| match f32::from_str(p) {
                    Ok(f) => f,
                    Err(e) => panic!("{}", e),
                })
                .unwrap_or(0.0);
            SpeciesRow::new(common_name, scientific_name, percent)
        })
        .collect::<Vec<_>>();
    let common_name = Series::new(
        COMMON_NAME,
        df_row.iter().map(|r| r.common_name).collect::<Vec<_>>(),
    );
    let scietific_name = Series::new(
        SCIENTIFIC_NAME,
        df_row.iter().map(|r| r.scientific_name).collect::<Vec<_>>(),
    );
    let percent = Series::new(
        PERCENT,
        df_row.iter().map(|r| r.percent).collect::<Vec<_>>(),
    );
    let checklist_column = Series::new(CHECKLISTS, vec![checklists; df_row.len()]);
    match DataFrame::new(vec![common_name, scietific_name, percent, checklist_column]) {
        Ok(df) => df,
        Err(e) => panic!("{}", e)
    }
}
