use crate::location::country::{Country, Region, SubRegion};
use crate::location::selectors::Selectors;
use crate::location::{COUNTRIES, REGIONS, SUBREGIONS};
use reqwest::blocking::Client;
use scraper::{ElementRef, Html};
use std::collections::HashSet;

fn get_html(client: &Client, url: &str) -> Html {
    match client.get(url).send().and_then(|respone| respone.text()) {
        Ok(text) => Html::parse_document(&text),
        Err(e) => panic!("{:?}", e),
    }
}

fn parse_row(row: &ElementRef) -> (String, String) {
    let url = row
        .value()
        .attr("href")
        .unwrap_or_else(|| panic!("No url for row : {:?}", row));
    let name = row
        .value()
        .attr("title")
        .unwrap_or_else(|| panic!("No name for row: {:?}", row))
        .to_string();
    let code = match url.split("/").collect::<Vec<_>>().last() {
        Some(&c) => c.to_owned(),
        None => panic!("Improperly formatted url for row: {:?}", row),
    };
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
    let html = get_html(client, COUNTRIES);
    html.select(&selectors.leaderboard)
        .next()
        .map(|element| {
            element
                .select(&selectors.a)
                .map(|row| parse_country(&row))
                .collect::<HashSet<_>>()
        })
        .unwrap_or(HashSet::new())
        .into_iter()
        .collect::<Vec<_>>()
}

pub fn get_regions<'a>(
    client: &Client,
    selectors: &Selectors,
    country: &'a Country,
) -> Vec<Region<'a>> {
    let region_url = format!("{}/{}/{}", REGIONS, country.country_code, SUBREGIONS);
    let html = get_html(client, &region_url);
    html.select(&selectors.leaderboard)
        .next()
        .map(|element| {
            element
                .select(&selectors.a)
                .map(|row| parse_region(&row, country))
                .collect::<HashSet<_>>()
        })
        .unwrap_or(HashSet::new())
        .into_iter()
        .collect::<Vec<_>>()
}

pub fn get_sub_regions<'a>(
    client: &Client,
    selectors: &Selectors,
    region: &'a Region,
) -> Vec<SubRegion<'a>> {
    let sub_region_url = format!("{}/{}/{}", REGIONS, region.region_code, SUBREGIONS);
    let html = get_html(client, &sub_region_url);
    html.select(&selectors.leaderboard)
        .next()
        .map(|element| {
            element
                .select(&selectors.a)
                .map(|row| parse_sub_region(&row, region))
                .collect::<HashSet<_>>()
        })
        .unwrap_or(HashSet::new())
        .into_iter()
        .collect::<Vec<_>>()
}
