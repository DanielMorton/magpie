use crate::error::Result;
use crate::selectors;
use scraper::ElementRef;

#[derive(Debug, Clone)]
pub struct SpeciesRecord {
    pub common_name: String,
    pub scientific_name: String,
    pub percent: f32,
    pub checklists: i32,
}

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

pub fn scrape_table(table: ElementRef, checklists: i32) -> Result<Vec<SpeciesRecord>> {
    let mut records = Vec::new();

    for row in table.select(selectors::target::rows()) {
        let (common, scientific) = get_species(&row)?;
        let percent = get_percent(&row);
        records.push(SpeciesRecord {
            common_name: common,
            scientific_name: scientific,
            percent,
            checklists,
        });
    }

    Ok(records)
}
