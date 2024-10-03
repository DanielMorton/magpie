use std::collections::HashSet;
use crate::location::loc::{Hotspot, SubRegion};
use crate::location::regions::{get_html, parse_row};
use crate::location::selectors::Selectors;
use crate::location::{HOTSPOT, REGIONS};
use reqwest::blocking::Client;
use scraper::ElementRef;
use std::thread;
use std::time::Duration;

fn parse_hotspot<'a>(
    row: &ElementRef,
    sub_region: &'a SubRegion,
) -> Result<Hotspot<'a>, &'static str> {
    let (hotspot, hotspot_code) = parse_row(row)?;
    Ok(Hotspot::new(&hotspot, &hotspot_code, sub_region))
}
pub fn get_hotspots<'a>(
    client: &Client,
    sub_region: &'a SubRegion,
    tries: u64,
) -> Vec<Hotspot<'a>> {
    let hotspot_url = format!("{}/{}/{}", REGIONS, sub_region.sub_region_code(), HOTSPOT);
    let html = match get_html(client, &hotspot_url) {
        Ok(html) => html,
        Err(e) => {
            eprintln!(
                "Error fetching hotspots for {}: {}",
                sub_region.sub_region(),
                e
            );
            return vec![];
        }
    };

    match html.select(Selectors::leaderboard()).next() {
        Some(element) => element
            .select(Selectors::a())
            .filter_map(|row| parse_hotspot(&row, sub_region).ok())
            .collect::<HashSet<_>>().into_iter().collect(),
        None => {
            thread::sleep(Duration::from_secs(tries));
            get_hotspots(client, sub_region, tries + 1)
        }
    }
}
