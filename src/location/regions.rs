use crate::location::loc::{Country, Region, SubRegion};
use crate::location::selectors::Selectors;
use crate::location::{COUNTRIES, REGIONS, SUBREGIONS};
use reqwest::blocking::Client;
use scraper::{ElementRef, Html};
use std::collections::HashSet;
use std::thread;
use std::time::Duration;

pub(crate) fn get_html(client: &Client, url: &str) -> Result<Html, reqwest::Error> {
    client
        .get(url)
        .send()?
        .text()
        .map(|text| Html::parse_document(&text))
}

pub(crate) fn parse_row(row: &ElementRef) -> Result<(String, String), &'static str> {
    let name = row.value().attr("title").ok_or("No name for row")?;
    let code = row
        .value()
        .attr("href")
        .ok_or("No url for row")?
        .split('/')
        .last()
        .ok_or("Improperly formatted url for row")?;
    Ok((name.to_owned(), code.to_owned()))
}

fn parse_country(row: &ElementRef) -> Result<Country, &'static str> {
    let (country, country_code) = parse_row(row)?;
    Ok(Country::new(&country, &country_code))
}

fn parse_region<'a>(row: &ElementRef, country: &'a Country) -> Result<Region<'a>, &'static str> {
    let (region, region_code) = parse_row(row)?;
    Ok(Region::new(&region, &region_code, country))
}

fn parse_sub_region<'a>(
    row: &ElementRef,
    region: &'a Region,
) -> Result<SubRegion<'a>, &'static str> {
    let (sub_region, sub_region_code) = parse_row(row)?;
    Ok(SubRegion::new(&sub_region, &sub_region_code, region))
}

pub fn get_countries(client: &Client) -> Result<Vec<Country>, reqwest::Error> {
    let html = get_html(client, COUNTRIES)?;
    Ok(html
        .select(Selectors::leaderboard())
        .next()
        .map(|element| {
            element
                .select(Selectors::a())
                .filter_map(|row| parse_country(&row).ok())
                .collect::<HashSet<_>>()
        })
        .unwrap_or_default()
        .into_iter()
        .collect())
}

pub fn get_regions<'a>(client: &Client, country: &'a Country, tries: u64) -> Vec<Region<'a>> {
    let region_url = format!("{}/{}/{}", REGIONS, country.country_code(), SUBREGIONS);
    let html = match get_html(client, &region_url) {
        Ok(html) => html,
        Err(e) => {
            eprintln!("Error fetching regions for {}: {}", country.country(), e);
            return vec![];
        }
    };

    match html.select(Selectors::leaderboard()).next() {
        Some(element) => {
            let regions: Vec<_> = element
                .select(Selectors::a())
                .filter_map(|row| parse_region(&row, country).ok())
                .collect::<HashSet<_>>()
                .into_iter()
                .collect();

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
        None => {
            thread::sleep(Duration::from_secs(tries));
            get_regions(client, country, tries + 1)
        }
    }
}

pub fn get_sub_regions<'a>(client: &Client, region: &'a Region, tries: u64) -> Vec<SubRegion<'a>> {
    let sub_region_url = format!("{}/{}/{}", REGIONS, region.region_code(), SUBREGIONS);
    let html = match get_html(client, &sub_region_url) {
        Ok(html) => html,
        Err(e) => {
            eprintln!("Error fetching sub-regions for {}: {}", region.region(), e);
            return vec![];
        }
    };

    match html.select(Selectors::leaderboard()).next() {
        Some(element) => {
            let sub_regions: Vec<_> = element
                .select(Selectors::a())
                .filter_map(|row| parse_sub_region(&row, region).ok())
                .collect::<HashSet<_>>()
                .into_iter()
                .collect();

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
        None => {
            thread::sleep(Duration::from_secs(tries));
            get_sub_regions(client, region, tries + 1)
        }
    }
}
