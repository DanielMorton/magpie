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
use std::sync::Arc;
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

    fn make_loc_vec(&self) -> Vec<LocationRow> {
        let loc_vec = if self.location_level == LocationLevel::Hotspot {
            HOTSPOT_COLUMNS
        } else {
            REGION_COLUMNS
        };
        let mut loc = self.loc_df
            .columns(loc_vec)
            .expect("Failed to get location columns")
            .iter()
            .map(|&s| s.iter())
            .collect::<Vec<_>>();
        (0..self.loc_df.shape().0)
            .map(|_| LocationRow::new(&mut loc))
            .collect()
    }

    fn make_loc_payload(&self) -> Vec<Vec<(String, String)>> {
        let location_level_code = self.location_level.to_string();
        let columns = if self.list_type == ListType::Global {
            vec![location_level_code]
        } else {
            vec![location_level_code, self.list_type.to_string()]
        };
        let mut col_iters = self.loc_df
            .columns(columns)
            .expect("Failed to get columns for payload")
            .iter()
            .map(|&s| s.iter())
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
        loc_payload
    }

    fn make_time_payload(&self) -> Vec<Vec<(String, u8)>> {
        self.time_range
            .iter()
            .map(|&(s, e)| vec![("bmo".to_string(), s), ("emo".to_string(), e)])
            .collect()
    }

    fn get_response(
        &self,
        loc: &[(String, String)],
        time: &[(String, u8)],
        date_query: &[(&str, String)],
        sleep: u64,
    ) -> Response {
        match self
            .client
            .get(BASE_URL)
            .query(loc)
            .query(time)
            .query(date_query)
            .send()
        {
            Ok(response) => {
                let url = response.url().to_string();
                if !(url.contains(LOGIN_URL) || url.contains(HOME_URL)) {
                    response
                } else {
                    thread::sleep(Duration::from_secs(sleep));
                    self.get_response(loc, time, date_query, 2 * sleep)
                }
            }
            Err(_) => {
                thread::sleep(Duration::from_secs(sleep));
                self.get_response(loc, time, date_query, 2 * sleep)
            }
        }
    }

    pub fn scrape_pages(&self) -> Result<DataFrame, PolarsError> {
        let date_query = Arc::new(vec![("t2", self.date_range.to_string())]);
        let loc_query = self.make_loc_payload();
        let loc_vec = self.make_loc_vec();
        let time_query = self.make_time_payload();
        let arc_scraper = Arc::new(self);

        let payloads: Vec<_> = loc_vec
            .into_iter()
            .zip(loc_query)
            .cartesian_product(time_query)
            .collect();

        let start = Instant::now();
        let style = ProgressStyle::with_template("{bar:100} {pos:>7}/{len:7} [{elapsed}] [{eta}]")
            .expect("Failed to create progress style");

        let output_list: Vec<_> = payloads
            .into_par_iter()
            .progress_with_style(style)
            .map(|((row, loc), time)| {
                let mut df = arc_scraper
                    .scrape_page(loc, &time, &date_query, MIN_BACKOFF)
                    .expect("Expected single table of data.");
                add_columns(&mut df, &row, &time).expect("Failed to add columns");
                df
            })
            .collect();

        print_hms(&start);
        concat_df_diagonal(&output_list)
    }

    fn scrape_page(
        &self,
        loc: Vec<(String, String)>,
        time: &[(String, u8)],
        date_query: &[(&str, String)],
        sleep: u64,
    ) -> Result<DataFrame, PolarsError> {
        let loc_code = &loc[0].1;
        let response = self.get_response(&loc, time, date_query, sleep);
        let url = response.url().to_string();
        let doc = match response.text() {
            Ok(text) => Html::parse_document(&text),
            Err(e) => {
                eprintln!("Error: {}", e);
                thread::sleep(Duration::from_secs(sleep));
                return self.scrape_page(loc, time, date_query, min(2 * sleep, MAX_BACKOFF));
            }
        };

        let (doc_selector, doc_format) = if self.location_level == LocationLevel::Hotspot {
            (Selectors::hotspot_select(), HOTSPOT)
        } else {
            (Selectors::region_select(), REGION)
        };

        if doc.select(doc_selector)
            .next()
            .and_then(|r| r.value().attr("href"))
            .filter(|&r| r == format!("{}/{}", doc_format, loc_code))
            .is_none()
        {
            return if sleep >= MAX_BACKOFF {
                eprintln!("Hotspot Empty {} {} {}", url, loc_code, sleep);
                empty_table()
            } else {
                thread::sleep(Duration::from_secs(sleep));
                self.scrape_page(loc, time, date_query, 2 * sleep)
            };
        }

        let checklists = doc
            .select(Selectors::checklists())
            .next()
            .and_then(|element| element.text().next())
            .and_then(|text| text.chars().filter(|c| c.is_numeric()).collect::<String>().parse().ok())
            .unwrap_or(0);

        match doc
            .select(Selectors::species_count())
            .next()
            .and_then(|count| count.text().next())
            .and_then(|count| u32::from_str(count).ok())
        {
            Some(0) => empty_table(),
            Some(_) => doc.select(Selectors::native()).next()
                .map_or_else(empty_table, |t| scrape_table(t, checklists)),
            None => {
                thread::sleep(Duration::from_secs(sleep));
                self.scrape_page(loc, time, date_query, min(2 * sleep, MAX_BACKOFF))
            }
        }
    }
}