use crate::error::Result;
use crate::selectors;
use polars::functions::concat_df_diagonal;
use polars::prelude::{Column, DataFrame, NamedFrom, Series};
use scraper::ElementRef;

fn get_common_name(species: Option<&scraper::ElementRef>) -> String {
    species
        .and_then(|s| s.text().next())
        .map(|s| s.trim().to_owned())
        .unwrap_or_default()
}

fn get_scientific_name(species: Option<&scraper::ElementRef>) -> String {
    species
        .and_then(|s| s.select(selectors::target::sci_name()).next())
        .and_then(|s| s.text().next())
        .map(|s| s.trim().to_owned())
        .unwrap_or_default()
}

fn get_species(row: &ElementRef) -> Result<(String, String)> {
    let species = row
        .select(selectors::target::species())
        .next()
        .and_then(|s| s.select(selectors::target::a()).next());
    Ok((
        get_common_name(species.as_ref()),
        get_scientific_name(species.as_ref()),
    ))
}

fn get_percent(row: &ElementRef) -> f32 {
    row.select(selectors::target::percent())
        .next()
        .and_then(|p| p.value().attr("title"))
        .and_then(|p| p.split('%').next())
        .and_then(|p| p.parse().ok())
        .unwrap_or(0.0)
}

pub fn scrape_table(table: ElementRef, checklists: i32) -> Result<DataFrame> {
    let rows: Result<Vec<DataFrame>> = table
        .select(selectors::target::rows())
        .map(|row| {
            let (common, scientific) = get_species(&row)?;
            let percent = get_percent(&row);
            DataFrame::new(
                1,
                vec![
                    Column::from(Series::new("common name".into(), vec![common])),
                    Column::from(Series::new("scientific name".into(), vec![scientific])),
                    Column::from(Series::new("percent".into(), vec![percent])),
                ],
            )
            .map_err(Into::into)
        })
        .collect();

    let mut df = concat_df_diagonal(&rows?)?;
    df.with_column(Column::from(Series::new(
        "checklists".into(),
        vec![checklists; df.height()],
    )))?;
    Ok(df)
}
