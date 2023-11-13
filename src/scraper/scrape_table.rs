use std::str::FromStr;
use std::sync::Arc;
use polars::prelude::{DataFrame, NamedFrom, Series};
use scraper::ElementRef;
use crate::row::SpeciesRow;
use crate::scraper::selectors::Selectors;

pub(super) fn scrape_table(selectors: &Arc<Selectors>,
                           table: ElementRef) -> DataFrame {
    let df_row = table
        .select(&selectors.rows())
        .map(|row| {
            let species = row
                .select(&selectors.species())
                .next()
                .map(|s| s.select(&selectors.a()).next())
                .flatten();
            let common_name = species
                .map(|s| s.text().next())
                .flatten()
                .unwrap_or("")
                .trim();
            let scientific_name = species
                .map(|s| {
                    s.select(&selectors.sci_name())
                        .next()
                        .map(|s| s.text().next())
                        .flatten()
                })
                .flatten()
                .unwrap_or("")
                .trim();
            let percent = row
                .select(&selectors.percent())
                .next()
                .map(|p| p.value().attr("title"))
                .flatten()
                .map(|p| p.split("%").next())
                .flatten()
                .map(|p| match f32::from_str(p) {
                    Ok(f) => f,
                    Err(e) => panic!("{}", e),
                })
                .unwrap_or(0.0);
            SpeciesRow::new(common_name, scientific_name, percent)
        })
        .collect::<Vec<_>>();
    let common_name = Series::new(
        "common name",
        df_row.iter().map(|r| r.common_name()).collect::<Vec<_>>(),
    );
    let scietific_name = Series::new(
        "scientific name",
        df_row
            .iter()
            .map(|r| r.scientific_name())
            .collect::<Vec<_>>(),
    );
    let percent = Series::new(
        "percent",
        df_row.iter().map(|r| r.percent()).collect::<Vec<_>>(),
    );
    return DataFrame::new(vec![common_name, scietific_name, percent]).unwrap();
}