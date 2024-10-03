use crate::location::loc::{Country, Region, SubRegion};
use crate::location::selectors::Selectors;
use crate::location::{COUNTRIES, REGIONS, SUBREGIONS};
use reqwest::blocking::Client;
use scraper::{ElementRef, Html};
use std::collections::HashSet;
use std::thread;
use std::time::Duration;

pub(crate) fn get_html(client: &Client, url: &str) -> Html {
    match client.get(url).send().and_then(|respone| respone.text()) {
        Ok(text) => Html::parse_document(&text),
        Err(e) => panic!("{:?}", e),
    }
}

pub(crate) fn parse_row(row: &ElementRef) -> (String, String) {
    let name = row
        .value()
        .attr("title")
        .unwrap_or_else(|| panic!("No name for row: {:?}", row))
        .to_owned();
    let code = row
        .value()
        .attr("href")
        .unwrap_or_else(|| panic!("No url for row : {:?}", row))
        .split('/')
        .collect::<Vec<_>>()
        .last()
        .unwrap_or_else(|| panic!("Improperly formatted url for row: {:?}", row))
        .to_owned()
        .to_owned();
    (name, code)
}
fn parse_country(row: &ElementRef) -> Country {
    let (country, country_code) = parse_row(row);
    Country::new(country, country_code)
}

fn parse_region<'a>(row: &ElementRef, country: &'a Country) -> Region<'a> {
    let (region, region_code) = parse_row(row);
    Region::new(region, region_code, country)
}

fn parse_sub_region<'a>(row: &ElementRef, region: &'a Region) -> SubRegion<'a> {
    let (sub_region, sub_region_code) = parse_row(row);
    SubRegion::new(sub_region, sub_region_code, region)
}

pub fn get_countries(client: &Client, selectors: &Selectors) -> Vec<Country> {
    get_html(client, COUNTRIES)
        .select(&selectors.leaderboard)
        .next()
        .map(|element| {
            element
                .select(&selectors.a)
                .map(|row| parse_country(&row))
                .collect::<HashSet<_>>()
        })
        .unwrap_or_default()
        .into_iter()
        .collect::<Vec<_>>()
}

pub fn get_regions<'a>(
    client: &Client,
    selectors: &Selectors,
    country: &'a Country,
    tries: u64
) -> Vec<Region<'a>> {
    let region_url = format!("{}/{}/{}", REGIONS, country.country_code, SUBREGIONS);
    let html = get_html(client, &region_url);
    let regions_leaderboard = html
        .select(&selectors.leaderboard)
        .next();
    if regions_leaderboard.is_none() {
        thread::sleep(Duration::from_secs(tries));
        return get_regions(client, selectors, country, tries + 1);
    }
    let regions = regions_leaderboard.map(|element| {
            element
                .select(&selectors.a)
                .map(|row| parse_region(&row, country))
                .collect::<HashSet<_>>()
        })
        .unwrap_or_default()
        .into_iter()
        .collect::<Vec<_>>();
    if !regions.is_empty() {
        regions
    } else {
        vec![Region::new(
            country.country(),
            country.country_code(),
            country,
        )]
    }
}

pub fn get_sub_regions<'a>(
    client: &Client,
    selectors: &Selectors,
    region: &'a Region,
    tries: u64
) -> Vec<SubRegion<'a>> {
    let sub_region_url = format!("{}/{}/{}", REGIONS, region.region_code, SUBREGIONS);
    let html = get_html(client, &sub_region_url);
    let sub_region_leaderboard = html
        .select(&selectors.leaderboard)
        .next();
    if sub_region_leaderboard.is_none() {
        thread::sleep(Duration::from_secs(tries));
        return get_sub_regions(client, selectors, region, tries + 1);
    }
    let sub_regions = sub_region_leaderboard.map(|element| {
            element
                .select(&selectors.a)
                .map(|row| parse_sub_region(&row, region))
                .collect::<HashSet<_>>()
        })
        .unwrap_or_default()
        .into_iter()
        .collect::<Vec<_>>();
    if !sub_regions.is_empty() {
        sub_regions
    } else {
        vec![SubRegion::new(
            region.region(),
            region.region_code(),
            region,
        )]
    }
}
