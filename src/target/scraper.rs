use crate::error::{AppError, Result};
use crate::selectors;
use crate::target::row::LocationRow;
use crate::target::scrape_params::{DateRange, ListType, LocationLevel};
use crate::target::table::{add_columns, empty_table};
use crate::utils::print_elapsed;
use indicatif::{MultiProgress, ParallelProgressIterator, ProgressBar, ProgressStyle};
use itertools::Itertools;
use polars::functions::concat_df_diagonal;
use polars::prelude::DataFrame;
use rayon::prelude::*;
use reqwest::blocking::Client;
use scraper::Html;
use std::cmp::min;
use std::thread;
use std::time::{Duration, Instant};
use tracing::warn;

const BASE_URL: &str = "https://ebird.org/targets";
const HOME_URL: &str = "https://ebird.org/home";
const LOGIN_URL: &str = "https://secure.birds.cornell.edu/cassso/login";
const MAX_BACKOFF_SECS: u64 = 100;
const MIN_BACKOFF_SECS: u64 = 5;

pub struct Scraper {
    client: Client,
    date_range: DateRange,
    location_level: LocationLevel,
    list_type: ListType,
    loc_df: DataFrame,
    time_ranges: Vec<(u8, u8)>,
}

impl Scraper {
    pub fn new(
        client: Client,
        date_range: DateRange,
        location_level: LocationLevel,
        list_type: ListType,
        loc_df: DataFrame,
        time_ranges: Vec<(u8, u8)>,
    ) -> Self {
        Self { client, date_range, location_level, list_type, loc_df, time_ranges }
    }

    fn make_loc_rows(&self) -> Result<Vec<LocationRow>> {
        let cols = match self.location_level {
            LocationLevel::Hotspot => &["country", "region", "sub_region", "hotspot"][..],
            LocationLevel::SubRegion => &["country", "region", "sub_region"][..],
        };

        let selected = self.loc_df.select(cols)?;
        let mut iters: Vec<_> = selected.columns()
            .iter()
            .map(|c| c.as_materialized_series().iter())
            .collect();

        (0..self.loc_df.shape().0)
            .map(|_| LocationRow::from_iters(&mut iters))
            .collect()
    }

    fn make_loc_payloads(&self) -> Result<Vec<Vec<(String, String)>>> {
        let level_code = match self.location_level {
            LocationLevel::Hotspot => "hotspot_code",
            LocationLevel::SubRegion => "sub_region_code",
        };

        let second_col = format!("{}_code", self.list_type);
        let cols: Vec<&str> = if self.list_type == ListType::Global {
            vec![level_code]
        } else {
            vec![level_code, &second_col]
        };

        let selected = self.loc_df.select(cols)?;
        let mut iters: Vec<_> = selected.columns()
            .iter()
            .map(|c| c.as_materialized_series().iter())
            .collect();

        let mut payloads: Vec<Vec<(String, String)>> = (0..self.loc_df.shape().0)
            .map(|_| {
                iters.iter_mut().enumerate()
                    .map(|(i, iter)| {
                        let val = iter.next().unwrap().to_string();
                        (format!("r{}", i + 1), val.trim_matches('"').to_owned())
                    })
                    .collect()
            })
            .collect();

        if self.list_type == ListType::Global {
            payloads.iter_mut().for_each(|p| p.push(("r2".into(), "world".into())));
        }
        Ok(payloads)
    }

    fn fetch_with_backoff(
        &self,
        loc: &[(String, String)],
        time: &[(String, u8)],
        date_query: &[(&str, String)],
        sleep_secs: u64,
    ) -> Result<reqwest::blocking::Response> {
        let response = self.client
            .get(BASE_URL)
            .query(loc)
            .query(time)
            .query(date_query)
            .send()?;

        let url = response.url().to_string();
        if url.contains(LOGIN_URL) || url.contains(HOME_URL) {
            if sleep_secs >= MAX_BACKOFF_SECS {
                return Err(AppError::MaxRetries(url));
            }
            thread::sleep(Duration::from_secs(sleep_secs));
            self.fetch_with_backoff(loc, time, date_query, min(sleep_secs * 2, MAX_BACKOFF_SECS))
        } else {
            Ok(response)
        }
    }

    fn scrape_single(
        &self,
        _row: LocationRow,
        loc: Vec<(String, String)>,
        time: Vec<(String, u8)>,
    ) -> Result<DataFrame> {
        let loc_code = loc[0].1.clone();
        let date_query = vec![("t2", self.date_range.to_string())];

        let response = self.fetch_with_backoff(&loc, &time, &date_query, MIN_BACKOFF_SECS)?;
        let url = response.url().to_string();
        let text = response.text()?;
        let doc = Html::parse_document(&text);

        let (selector, format) = match self.location_level {
            LocationLevel::Hotspot => (selectors::target::hotspot_select(), "hotspot"),
            LocationLevel::SubRegion => (selectors::target::region_select(), "region"),
        };

        let valid = doc.select(selector)
            .next()
            .and_then(|r| r.value().attr("href"))
            .map(|href| href == format!("{}/{}", format, loc_code))
            .unwrap_or(false);

        if !valid {
            warn!("Page validation failed for {}: {}", loc_code, url);
            return empty_table();
        }

        let checklists = doc.select(selectors::target::checklists())
            .next()
            .and_then(|el| el.text().next())
            .and_then(|text| text.chars().filter(|c| c.is_numeric()).collect::<String>().parse().ok())
            .unwrap_or(0);

        let species_count = doc.select(selectors::target::species_count())
            .next()
            .and_then(|c| c.text().next())
            .and_then(|c| c.parse::<u32>().ok());

        match species_count {
            Some(0) | None => empty_table(),
            Some(_) => {
                doc.select(selectors::target::native())
                    .next()
                    .map_or_else(empty_table, |t| crate::target::scrape_table::scrape_table(t, checklists))
            }
        }
    }

    pub fn scrape_all(&self) -> Result<DataFrame> {
        let start = Instant::now();
        let loc_rows = self.make_loc_rows()?;
        let loc_payloads = self.make_loc_payloads()?;

        let items: Vec<_> = loc_rows.into_iter()
            .zip(loc_payloads)
            .cartesian_product(self.time_ranges.clone().into_iter().map(|(s, e)| {
                vec![("bmo".into(), s), ("emo".into(), e)]
            }))
            .collect();

        let total = items.len();
        let mp = MultiProgress::new();
        let pb = mp.add(ProgressBar::new(total as u64));
        pb.set_style(
            ProgressStyle::with_template("{bar:60.green} {pos}/{len} [{elapsed}] {msg}")
                .unwrap()
                .progress_chars("=> "),
        );
        pb.set_message("Scraping");

        let all_dfs: Vec<_> = items
            .into_par_iter()
            .progress_with(pb.clone())
            .filter_map(|((row, loc), time)| {
                match self.scrape_single(row.clone(), loc, time.clone()) {
                    Ok(mut df) => {
                        if add_columns(&mut df, &row, &time).is_ok() {
                            Some(df)
                        } else {
                            warn!("Failed to add columns for {}", row.country);
                            None
                        }
                    }
                    Err(e) => {
                        warn!("Scrape failed for {}: {}", row.country, e);
                        None
                    }
                }
            })
            .collect();

        pb.finish_with_message("Done");
        print_elapsed(&start, "Scraping complete");

        if all_dfs.is_empty() {
            empty_table()
        } else {
            concat_df_diagonal(&all_dfs).map_err(Into::into)
        }
    }
}