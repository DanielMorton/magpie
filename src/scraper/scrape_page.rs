use crate::scraper::row::LocationRow;
use crate::scraper::scrape_params::LocationLevel;
use crate::scraper::scrape_table::scrape_table;
use crate::scraper::scraper::Scraper;
use crate::scraper::selectors::Selectors;
use crate::scraper::table::{add_columns, empty_table};
use crate::scraper::utils::print_hms;
use crate::scraper::{HOTSPOT, MAX_BACKOFF, MIN_BACKOFF, REGION};
use itertools::Itertools;
use polars::functions::concat_df_diagonal;
use polars::prelude::DataFrame;
use rayon::prelude::*;
use scraper::{Html, Selector};
use std::cmp::min;
use std::str::FromStr;
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

/**
 Scrapes species frequency data from a single page. Checks if correct URL is returned.
 If the incorrect page is returned, retries after a delay. Delay time doubles after each retry.
 Only returns data about native and naturalized species, exotics and escapees are discarded.
 If no data for location and time parameters, returns an empty table.
 */
fn scrape_page(
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
    match doc
        .select(selectors.species_count())
        .next()
        .and_then(|count| count.text().next())
        .and_then(|count| u32::from_str(count).ok())
    {
        Some(0) => empty_table(),
        Some(_) => match doc.select(selectors.native()).next() {
            Some(t) => scrape_table(selectors, t),
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


pub fn scrape_pages(scraper: Scraper) -> DataFrame {
    let date_query = Arc::new(vec![("t2", scraper.date_range.to_string())]);
    let selectors = Arc::new(Selectors::new());
    let (doc_selector, doc_format) = if scraper.location_level == LocationLevel::Hotspot {
        (selectors.hotspot_select(), HOTSPOT)
    } else {
        (selectors.region_select(), REGION)
    };
    let loc_query = scraper.make_loc_payload();
    let loc_vec = scraper.make_loc_vec();
    let time_query = scraper.make_time_payload();
    let arc_scraper = Arc::new(scraper);
    let loc_payload = loc_vec
        .into_iter()
        .zip(loc_query)
        .collect::<Vec<(LocationRow, Vec<(String, String)>)>>();
    let payloads = loc_payload
        .into_iter()
        .cartesian_product(time_query)
        .collect::<Vec<_>>();
    let s = Instant::now();
    let output_list = payloads
        .into_par_iter()
        .map(|((row, loc), time)| {
            let mut df = scrape_page(
                &arc_scraper,
                &selectors,
                doc_selector,
                loc,
                &time,
                &date_query,
                doc_format,
                MIN_BACKOFF,
            );
            add_columns(&mut df, &row, &time);
            df
        })
        .collect::<Vec<_>>();

    print_hms(&s);
    match concat_df_diagonal(&output_list) {
        Ok(df) => df,
        Err(e) => panic!("{}", e),
    }
}
