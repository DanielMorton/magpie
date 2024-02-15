use crate::scraper::scrape_table::scrape_table;
use crate::scraper::scraper::Scraper;
use crate::scraper::selectors::Selectors;
use crate::scraper::table::empty_table;
use crate::scraper::MAX_BACKOFF;
use polars::prelude::DataFrame;
use scraper::{Html, Selector};
use std::cmp::min;
use std::str::FromStr;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

/**
Scrapes species frequency data from a single page. Checks if correct URL is returned.
If the incorrect page is returned, retries after a delay. Delay time doubles after each retry.
Only returns data about native and naturalized species, exotics and escapees are discarded.
If no data for location and time parameters, returns an empty table.
*/
pub(super) fn scrape_page(
    scraper: &Arc<Scraper>,
    selectors: &Arc<Selectors>,
    doc_selector: &Selector,
    loc: Vec<(String, String)>,
    time: &Vec<(String, u8)>,
    date_query: &Vec<(&str, String)>,
    doc_format: &str,
    sleep: u64,
) -> DataFrame {
    let loc_code = &loc[0].1;
    let response = scraper.get_response(&loc, time, date_query, sleep);
    //let url = response.url().to_string();
    let doc = match response.text() {
        Ok(text) => Html::parse_document(&text),
        Err(e) => {
            println!("{}", e);
            thread::sleep(Duration::from_secs(sleep));
            return scrape_page(
                scraper,
                selectors,
                doc_selector,
                loc,
                time,
                date_query,
                doc_format,
                min(2 * sleep, MAX_BACKOFF),
            );
        }
    };
    match doc
        .select(doc_selector)
        .next()
        .map(|r| r.value().attr("href").unwrap())
        .filter(|&r| r == format!("{}/{}", doc_format, loc_code))
    {
        Some(_) => (),
        None => {
            thread::sleep(Duration::from_secs(sleep));
            return scrape_page(
                scraper,
                selectors,
                doc_selector,
                loc,
                time,
                date_query,
                doc_format,
                min(2 * sleep, MAX_BACKOFF),
            );
            /* return if sleep >= MAX_BACKOFF {
                println!("Hotspot Empty {} {} {}", url, loc_code, &sleep);
                empty_table()
            } else {
                thread::sleep(Duration::from_secs(sleep));
                scrape_page(scraper, selectors, loc, time, date_query, 2 * sleep)
            }*/
        }
    }
    let checklists = doc
        .select(&selectors.checklists)
        .next()
        .and_then(|element| element.text().next())
        .map(|text| text.chars().filter(|c| c.is_numeric()).collect::<String>())
        .and_then(|c| c.parse::<i32>().ok())
        .unwrap_or(0);
    match doc
        .select(&selectors.species_count)
        .next()
        .and_then(|count| count.text().next())
        .and_then(|count| u32::from_str(count).ok())
    {
        Some(0) => empty_table(),
        Some(_) => match doc.select(&selectors.native).next() {
            Some(t) => scrape_table(selectors, t, checklists),
            None => empty_table(),
        },
        None => {
            thread::sleep(Duration::from_secs(sleep));
            scrape_page(
                scraper,
                selectors,
                doc_selector,
                loc,
                time,
                date_query,
                doc_format,
                min(2 * sleep, MAX_BACKOFF),
            )
        }
    }
}
