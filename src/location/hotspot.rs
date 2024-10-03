use crate::location::loc::{Hotspot, SubRegion};
use crate::location::regions::{get_html, parse_row};
use crate::location::selectors::Selectors;
use crate::location::{HOTSPOT, REGIONS};
use reqwest::blocking::Client;
use scraper::ElementRef;
use std::collections::HashSet;
use std::thread;
use std::time::Duration;

fn parse_hotspot<'a>(row: &ElementRef, sub_region: &'a SubRegion) -> Hotspot<'a> {
    let (hotspot, hotspot_code) = parse_row(row);
    Hotspot::new(hotspot, hotspot_code, sub_region)
}
pub fn get_hotspots<'a>(
    client: &Client,
    sub_region: &'a SubRegion,
    tries: u64,
) -> Vec<Hotspot<'a>> {
    let hotspot_url = format!("{}/{}/{}", REGIONS, sub_region.sub_region_code, HOTSPOT);
    let html = get_html(client, &hotspot_url);
    let hotspot_leaderboard = html.select(Selectors::leaderboard()).next();
    if hotspot_leaderboard.is_none() {
        thread::sleep(Duration::from_secs(tries));
        return get_hotspots(client, sub_region, tries + 1);
    }
    hotspot_leaderboard
        .map(|element| {
            element
                .select(Selectors::a())
                .map(|row| parse_hotspot(&row, sub_region))
                .collect::<HashSet<_>>()
        })
        .unwrap_or_default()
        .into_iter()
        .collect::<Vec<_>>()
}
