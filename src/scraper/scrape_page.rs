use crate::print_hms;
use crate::row::LocationRow;
use crate::scrape_params::ListLevel;
use crate::scraper::scrape_table::scrape_table;
use crate::scraper::scraper::Scraper;
use crate::scraper::selectors::Selectors;
use crate::table::{add_columns, empty_table};
use itertools::Itertools;
use polars::prelude::DataFrame;
use rayon::prelude::*;
use scraper::Html;
use std::str::FromStr;
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

static HOTSPOT: &str = "hotspot";
static MIN_BACKOFF: u64 = 5;
static REGION: &str = "region";

fn scrape_page(
    scraper: &Arc<Scraper>,
    selectors: &Arc<Selectors>,
    loc: Vec<(String, String)>,
    time: &Vec<(String, u8)>,
    date_query: &Vec<(&str, String)>,
    sleep: u64,
) -> DataFrame {
    let loc_code = &loc[0].1;
    let response = scraper.get_response(&loc, &time, date_query, sleep);
    let url = response.url().to_string();
    let doc = match response.text() {
        Ok(text) => Html::parse_document(&text),
        Err(e) => {
            println!("{}", e);
            //println!("HTML Empty {} {} {}", url, loc_code, &sleep);
            thread::sleep(Duration::from_secs(sleep));
            return scrape_page(scraper, selectors, loc, time, date_query, 2 * sleep);
        }
    };
    let (doc_selector, doc_format) = if scraper.list_level == ListLevel::Hotspot {
        (selectors.hotspot_select(), HOTSPOT)
    } else {
        (selectors.region_select(), REGION)
    };
    match doc
        .select(doc_selector)
        .next()
        .map(|r| r.value().attr("href").unwrap())
        .filter(|&r| r == format!("{}/{}", doc_format, loc_code))
    {
        Some(_) => (),
        None => {
            if sleep > 200 {
                println!("Hotspot Empty {} {} {}", url, loc_code, &sleep);
                return empty_table();
            }
            thread::sleep(Duration::from_secs(sleep));
            return scrape_page(scraper, selectors, loc, time, date_query, 2 * sleep);
        }
    }
    match doc
        .select(&selectors.species_count())
        .next()
        .map(|count| count.text().next())
        .flatten()
        .map(|count| u32::from_str(count).ok())
        .flatten()
    {
        Some(0) => empty_table(),
        Some(_) => match doc.select(&selectors.native()).next() {
            Some(t) => scrape_table(selectors, t),
            None => empty_table(),
        },
        None => {
            //println!("Doc Empty {} {} {}", url, loc_code, &sleep);
            thread::sleep(Duration::from_secs(sleep));
            scrape_page(scraper, selectors, loc, time, date_query, 2 * sleep)
        }
    }
}

pub fn scrape_pages(scraper: Scraper) -> DataFrame {
    let date_query = Arc::new(vec![("t2", scraper.date_range.to_string())]);
    let selectors = Arc::new(Selectors::new());
    let loc_query = scraper.make_loc_payload();
    let loc_vec = scraper.make_loc_vec();
    let time_query = scraper.make_time_payload();
    let arc_scraper = Arc::new(scraper);
    let loc_payload = loc_vec
        .into_iter()
        .zip(loc_query.into_iter())
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
                loc,
                &time,
                &date_query,
                MIN_BACKOFF,
            );
            add_columns(&mut df, &row, &time);
            df
        })
        .collect::<Vec<_>>();

    print_hms(&s);
    output_list
        .into_iter()
        .reduce(|a, b| a.vstack(&b).unwrap())
        .unwrap()
}
