use crate::location::location::{Hotspot, SubRegion};
use crate::location::regions::{get_html, parse_row};
use crate::location::selectors::Selectors;
use crate::location::{HOTSPOT, REGIONS};
use reqwest::blocking::Client;
use scraper::ElementRef;
use std::collections::HashSet;

fn parse_hotspot<'a>(row: &ElementRef, sub_region: &'a SubRegion) -> Hotspot<'a> {
    let (hotspot, hotspot_code) = parse_row(row);
    Hotspot::new(hotspot, hotspot_code, sub_region)
}
pub fn get_hotspots<'a>(
    client: &Client,
    selectors: &Selectors,
    sub_region: &'a SubRegion,
) -> Vec<Hotspot<'a>> {
    let hotspot_url = format!("{}/{}/{}", REGIONS, sub_region.sub_region_code, HOTSPOT);
    let html = get_html(client, &hotspot_url);
    html.select(&selectors.leaderboard)
        .next()
        .map(|element| {
            element
                .select(&selectors.a)
                .map(|row| parse_hotspot(&row, sub_region))
                .collect::<HashSet<_>>()
        })
        .unwrap_or_default()
        .into_iter()
        .collect::<Vec<_>>()
}
