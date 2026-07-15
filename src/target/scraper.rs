use crate::target::error::LocationError;
use crate::target::row::LocationRow;
use crate::target::scrape_params::{DateRange, ListType, LocationLevel};
use crate::target::scrape_table::scrape_table;
use crate::target::selectors::Selectors;
use crate::target::table::{add_columns, empty_table};
use crate::target::utils::{print_hms, remove_quote};
use crate::target::{
    BASE_URL, HOME_URL, HOTSPOT, HOTSPOT_COLUMNS, LOGIN_URL, MAX_BACKOFF, MIN_BACKOFF, REGION,
    REGION_COLUMNS,
};
use indicatif::{ParallelProgressIterator, ProgressStyle};
use itertools::Itertools;
use polars::functions::concat_df_diagonal;
use polars::prelude::{DataFrame, PolarsError};
use rayon::prelude::*;
use reqwest::blocking::{Client, Response};
use scraper::Html;
use std::cmp::min;
use std::str::FromStr;
use std::thread;
use std::time::{Duration, Instant};

pub struct Scraper {
    client: Client,
    pub(super) date_range: DateRange,
    pub(super) location_level: LocationLevel,
    list_type: ListType,
    loc_df: DataFrame,
    time_range: Vec<(u8, u8)>,
}

/// Doubles `sleep`, capped at `MAX_BACKOFF`, for the next retry iteration.
fn next_backoff(sleep: u64) -> u64 {
    min(sleep.saturating_mul(2), MAX_BACKOFF)
}

impl Scraper {
    pub(crate) fn new(
        client: Client,
        date_range: DateRange,
        list_level: LocationLevel,
        list_type: ListType,
        loc_df: DataFrame,
        time_range: Vec<(u8, u8)>,
    ) -> Self {
        Self {
            client,
            date_range,
            location_level: list_level,
            list_type,
            loc_df,
            time_range,
        }
    }

    fn make_loc_vec(&self) -> Result<Vec<LocationRow>, LocationError> {
        let loc_vec = if self.location_level == LocationLevel::Hotspot {
            HOTSPOT_COLUMNS
        } else {
            REGION_COLUMNS
        };
        let selected = self
            .loc_df
            .select(loc_vec)
            .expect("Failed to get location columns");
        let mut loc = selected
            .columns()
            .iter()
            .map(|col| col.as_materialized_series().iter())
            .collect::<Vec<_>>();
        (0..self.loc_df.shape().0)
            .map(|_| LocationRow::new(&mut loc))
            .collect::<Result<Vec<_>, _>>()
    }

    fn make_loc_payload(&self) -> Result<Vec<Vec<(String, String)>>, PolarsError> {
        let location_level_code = self.location_level.to_string();
        let columns = if self.list_type == ListType::Global {
            vec![location_level_code]
        } else {
            vec![location_level_code, self.list_type.to_string()]
        };
        let selected = self.loc_df.select(columns)?;
        let mut col_iters = selected
            .columns()
            .iter()
            .map(|col| col.as_materialized_series().iter())
            .collect::<Vec<_>>();

        let mut loc_payload: Vec<Vec<(String, String)>> = (0..self.loc_df.shape().0)
            .map(|_| {
                col_iters
                    .iter_mut()
                    .enumerate()
                    .map(|(i, iter)| {
                        let value = iter.next().unwrap().to_string();
                        (format!("r{}", i + 1), remove_quote(&value))
                    })
                    .collect()
            })
            .collect();

        if self.list_type == ListType::Global {
            loc_payload.iter_mut().for_each(|payload| {
                payload.push(("r2".to_string(), "world".to_string()));
            });
        }
        Ok(loc_payload)
    }

    fn make_time_payload(&self) -> Result<Vec<Vec<(String, u8)>>, PolarsError> {
        Ok(self
            .time_range
            .iter()
            .map(|&(s, e)| vec![("bmo".to_string(), s), ("emo".to_string(), e)])
            .collect())
    }

    /// Requests a page, retrying with capped exponential backoff whenever the request fails
    /// outright or eBird bounces us to the login/home page (an expired-session signal).
    /// This loops rather than recursing so an unstable connection can't grow the call stack
    /// without bound.
    fn get_response(
        &self,
        loc: &[(String, String)],
        time: &[(String, u8)],
        date_query: &[(&str, String)],
        mut sleep: u64,
    ) -> Response {
        loop {
            if let Ok(response) = self
                .client
                .get(BASE_URL)
                .query(loc)
                .query(time)
                .query(date_query)
                .send()
            {
                let url = response.url().to_string();
                if !(url.contains(LOGIN_URL) || url.contains(HOME_URL)) {
                    return response;
                }
            }
            thread::sleep(Duration::from_secs(sleep));
            sleep = next_backoff(sleep);
        }
    }

    pub fn scrape_pages(&self) -> Result<DataFrame, PolarsError> {
        let date_query = vec![("t2", self.date_range.to_string())];
        let loc_query = self.make_loc_payload()?;
        let loc_vec = self.make_loc_vec()?;
        let time_query = self.make_time_payload()?;

        let payloads: Vec<_> = loc_vec
            .into_iter()
            .zip(loc_query)
            .cartesian_product(time_query)
            .collect();

        let start = Instant::now();
        let style = ProgressStyle::with_template("{bar:100} {pos:>7}/{len:7} [{elapsed}] [{eta}]")
            .expect("Failed to create progress style");

        // `self` is a plain reference, which is `Copy`; no `Arc` wrapping is needed for rayon
        // to share it across worker threads.
        let output_list: Result<Vec<DataFrame>, PolarsError> = payloads
            .into_par_iter()
            .progress_with_style(style)
            .map(|((row, loc), time)| {
                let mut df = self.scrape_page(loc, &time, &date_query, MIN_BACKOFF)?;
                add_columns(&mut df, &row, &time)?;
                Ok(df)
            })
            .collect();

        print_hms(&start);
        concat_df_diagonal(&output_list?)
    }

    /// Scrapes a single target-species page, retrying with capped exponential backoff on
    /// transient failures (network errors, an unrecognized/not-yet-ready response, or a
    /// checklist count that hasn't loaded yet). Looping instead of recursing keeps retries
    /// from growing the call stack even when a location proves persistently uncooperative.
    fn scrape_page(
        &self,
        loc: Vec<(String, String)>,
        time: &[(String, u8)],
        date_query: &[(&str, String)],
        mut sleep: u64,
    ) -> Result<DataFrame, PolarsError> {
        let loc_code = &loc[0].1;
        let (doc_selector, doc_format) = if self.location_level == LocationLevel::Hotspot {
            (Selectors::hotspot_select(), HOTSPOT)
        } else {
            (Selectors::region_select(), REGION)
        };
        let expected_href = format!("{}/{}", doc_format, loc_code);

        loop {
            let response = self.get_response(&loc, time, date_query, sleep);
            let url = response.url().to_string();
            let doc = match response.text() {
                Ok(text) => Html::parse_document(&text),
                Err(e) => {
                    eprintln!("Error: {}", e);
                    thread::sleep(Duration::from_secs(sleep));
                    sleep = next_backoff(sleep);
                    continue;
                }
            };

            let location_confirmed = doc
                .select(doc_selector)
                .next()
                .and_then(|r| r.value().attr("href"))
                .is_some_and(|r| r == expected_href);

            if !location_confirmed {
                if sleep >= MAX_BACKOFF {
                    eprintln!("Hotspot Empty {} {} {}", url, loc_code, sleep);
                    return empty_table();
                }
                thread::sleep(Duration::from_secs(sleep));
                sleep = next_backoff(sleep);
                continue;
            }

            let checklists = doc
                .select(Selectors::checklists())
                .next()
                .and_then(|element| element.text().next())
                .and_then(|text| {
                    text.chars()
                        .filter(|c| c.is_numeric())
                        .collect::<String>()
                        .parse()
                        .ok()
                })
                .unwrap_or(0);

            match doc
                .select(Selectors::species_count())
                .next()
                .and_then(|count| count.text().next())
                .and_then(|count| u32::from_str(count).ok())
            {
                Some(0) => return empty_table(),
                Some(_) => {
                    return doc
                        .select(Selectors::native())
                        .next()
                        .map_or_else(empty_table, |t| scrape_table(t, checklists));
                }
                None => {
                    thread::sleep(Duration::from_secs(sleep));
                    sleep = next_backoff(sleep);
                }
            }
        }
    }
}
