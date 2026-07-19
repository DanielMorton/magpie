use crate::error::{AppError, Result};
use crate::location::loc::{Country, Hotspot, Region, SubRegion};
use crate::selectors;
use reqwest::blocking::Client;
use scraper::{ElementRef, Html};
use std::collections::HashSet;
use std::hash::Hash;
use std::thread;
use std::time::Duration;
use tracing::warn;

const COUNTRIES_URL: &str = "https://ebird.org/region/world/subregions";
const REGIONS_BASE: &str = "https://ebird.org/region";
const SUBREGIONS_PATH: &str = "subregions";
const HOTSPOT_PATH: &str = "hotspots";

fn get_html(client: &Client, url: &str) -> Result<Html> {
    client
        .get(url)
        .send()?
        .text()
        .map(|text| Html::parse_document(&text))
        .map_err(Into::into)
}

fn parse_row(row: &ElementRef) -> Result<(String, String)> {
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

fn fetch_children<T, F>(
    client: &Client,
    parent_code: &str,
    path: &str,
    parser: F,
    fallback: impl FnOnce() -> Vec<T>,
    use_fallback_on_empty: bool,
) -> Vec<T>
where
    T: Hash + Eq,
    F: Fn((String, String)) -> Option<T>,
{
    let url = format!("{}/{}/{}", REGIONS_BASE, parent_code, path);

    let html = match get_html(client, &url) {
        Ok(html) => html,
        Err(e) => {
            warn!("Error fetching {}: {}", url, e);
            return vec![];
        }
    };

    match html.select(selectors::location::leaderboard()).next() {
        Some(element) => {
            let items: HashSet<_> = element
                .select(selectors::location::a())
                .filter_map(|row| parse_row(&row).ok())
                .filter_map(&parser)
                .collect();

            if !items.is_empty() {
                items.into_iter().collect()
            } else if use_fallback_on_empty {
                fallback()
            } else {
                vec![]
            }
        }
        None => {
            thread::sleep(Duration::from_secs(1));
            fetch_children(
                client,
                parent_code,
                path,
                parser,
                fallback,
                use_fallback_on_empty,
            )
        }
    }
}

pub fn get_countries(client: &Client) -> Result<Vec<Country>> {
    let html = get_html(client, COUNTRIES_URL)?;
    Ok(html
        .select(selectors::location::leaderboard())
        .next()
        .map(|element| {
            element
                .select(selectors::location::a())
                .filter_map(|row| {
                    let (name, code) = parse_row(&row).ok()?;
                    Some(Country::new(name, code))
                })
                .collect::<HashSet<_>>()
        })
        .unwrap_or_default()
        .into_iter()
        .collect())
}

pub fn get_regions<'a>(client: &Client, country: &'a Country) -> Vec<Region<'a>> {
    let cref = country;
    fetch_children(
        client,
        country.code(),
        SUBREGIONS_PATH,
        |(name, code)| Some(Region::new(name, code, cref)),
        || vec![Region::new(country.name(), country.code(), country)],
        true,
    )
}

pub fn get_sub_regions<'a>(client: &Client, region: &'a Region<'a>) -> Vec<SubRegion<'a>> {
    let rref = region;
    fetch_children(
        client,
        region.code(),
        SUBREGIONS_PATH,
        |(name, code)| Some(SubRegion::new(name, code, rref)),
        || vec![SubRegion::new(region.name(), region.code(), region)],
        true,
    )
}

pub fn get_hotspots<'a>(client: &Client, sub_region: &'a SubRegion<'a>) -> Vec<Hotspot<'a>> {
    let sref = sub_region;
    fetch_children(
        client,
        sub_region.code(),
        HOTSPOT_PATH,
        |(name, code)| Some(Hotspot::new(name, code, sref)),
        Vec::new,
        false,
    )
}
