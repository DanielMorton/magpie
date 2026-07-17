use crate::error::{AppError, Result};
use crate::location::loc::{Country, Hotspot, Region, SubRegion};
use crate::selectors;
use reqwest::Client;
use scraper::{ElementRef, Html};
use std::collections::HashSet;
use std::time::Duration;
use tokio::time::sleep;
use tracing::warn;

const COUNTRIES_URL: &str = "https://ebird.org/region/world/subregions";
const REGIONS_BASE: &str = "https://ebird.org/region";
const SUBREGIONS_PATH: &str = "subregions";
const HOTSPOT_PATH: &str = "hotspots";
const MAX_RETRIES: u32 = 5;
const BASE_DELAY_MS: u64 = 500;

async fn fetch_html(client: &Client, url: &str) -> Result<Html> {
    let mut delay = BASE_DELAY_MS;
    for attempt in 0..MAX_RETRIES {
        match client.get(url).send().await {
            Ok(response) => {
                let text = response.text().await?;
                return Ok(Html::parse_document(&text));
            }
            Err(e) if attempt < MAX_RETRIES - 1 => {
                warn!("Request failed (attempt {}): {}, retrying in {}ms", attempt + 1, e, delay);
                sleep(Duration::from_millis(delay)).await;
                delay *= 2;
            }
            Err(e) => return Err(e.into()),
        }
    }
    Err(AppError::MaxRetries(url.to_string()))
}

fn parse_row(row: &ElementRef) -> Option<(String, String)> {
    let name = row.value().attr("title")?;
    let code = row.value().attr("href")?.split('/').last()?;
    Some((name.to_owned(), code.to_owned()))
}

async fn fetch_children<T, F>(
    client: &Client,
    parent_code: &str,
    path: &str,
    parser: F,
    fallback: impl FnOnce() -> Vec<T>,
) -> Vec<T>
where
    F: Fn((String, String)) -> Option<T>,
{
    let url = format!("{}/{}/{}", REGIONS_BASE, parent_code, path);
    match fetch_html(client, &url).await {
        Ok(html) => {
            let items: HashSet<_> = html
                .select(selectors::location::leaderboard())
                .next()
                .into_iter()
                .flat_map(|el| el.select(selectors::location::a()))
                .filter_map(|row| parse_row(&row))
                .filter_map(&parser)
                .collect();
            if items.is_empty() { fallback() } else { items.into_iter().collect() }
        }
        Err(e) => {
            warn!("Failed to fetch {}: {}", url, e);
            fallback()
        }
    }
}

pub async fn get_countries(client: &Client) -> Result<Vec<Country>> {
    let html = fetch_html(client, COUNTRIES_URL).await?;
    Ok(html
        .select(selectors::location::leaderboard())
        .next()
        .into_iter()
        .flat_map(|el| el.select(selectors::location::a()))
        .filter_map(|row| parse_row(&row))
        .map(|(name, code)| Country::new(name, code))
        .collect::<HashSet<_>>()
        .into_iter()
        .collect())
}

pub async fn get_regions<'a>(client: &Client, country: &'a Country) -> Vec<Region<'a>> {
    let cref = country;
    fetch_children(
        client, country.code(), SUBREGIONS_PATH,
        |(name, code)| Some(Region::new(name, code, cref)),
        || vec![Region::new(country.name(), country.code(), country)],
    ).await
}

pub async fn get_sub_regions<'a>(client: &Client, region: &'a Region<'a>) -> Vec<SubRegion<'a>> {
    let rref = region;
    fetch_children(
        client, region.code(), SUBREGIONS_PATH,
        |(name, code)| Some(SubRegion::new(name, code, rref)),
        || vec![SubRegion::new(region.name(), region.code(), region)],
    ).await
}

pub async fn get_hotspots<'a>(client: &Client, sub_region: &'a SubRegion<'a>) -> Vec<Hotspot<'a>> {
    let sref = sub_region;
    fetch_children(
        client, sub_region.code(), HOTSPOT_PATH,
        |(name, code)| Some(Hotspot::new(name, code, sref)),
        Vec::new,
    ).await
}