use crate::scraper::row::LocationRow;
use crate::scraper::scrape_page::scrape_page;
use crate::scraper::scrape_params::{DateRange, ListType, LocationLevel};
use crate::scraper::selectors::Selectors;
use crate::scraper::table::add_columns;
use crate::scraper::utils::{print_hms, remove_quote};
use crate::scraper::{
    BASE_URL, HOME_URL, HOTSPOT, HOTSPOT_COLUMNS, LOGIN_URL, MIN_BACKOFF, REGION, REGION_COLUMNS,
};
use indicatif::{ParallelProgressIterator, ProgressStyle};
use itertools::Itertools;
use polars::functions::concat_df_diagonal;
use polars::prelude::DataFrame;
use rayon::prelude::*;
use reqwest::blocking::{Client, Response};
use std::ops::Deref;
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

/**
Struct containing Client and all data needed for scraping a set of pages. The client performs the
page requests.

The date_range refers to the temporal type of list for which the target species are extracted. Options are life list,
year list, month list, or day list.

The location level is spacial granularity of the location. Options are sub-region or hotspot.

List Type is the spacial type of the list for which target species are extracted. Options are global, country,
region, subregion, and hotspot.

loc_df is the DataFrame containing the location data.

Time range is the vector of time ranges, which may be single entry, in months for which the target species
are extracted. For each pair of start month and end month (which may be equal) only species present in a
given location between start month and end month inclusive are extracted.
*/
pub struct Scraper {
    /// Client for making page requests.
    client: Client,

    /// Time range of list type for which target species are extracted.
    pub(super) date_range: DateRange,

    /// Type of location, hotspot or sub-region, for which target species are extracted.
    pub(super) location_level: LocationLevel,

    /// Typee of list for which target species are extracted.
    list_type: ListType,

    /// DataFrame of locations for which data is extracted.
    loc_df: DataFrame,

    /// Vector of time ranges for which data is extracted.
    time_range: Vec<(u8, u8)>,
}

/// Implementation of Scraper struct.
impl Scraper {
    /// Constructor of Scraper struct from it's constituent components.
    pub(crate) fn new(
        client: Client,
        date_range: DateRange,
        list_level: LocationLevel,
        list_type: ListType,
        loc_df: DataFrame,
        time_range: Vec<(u8, u8)>,
    ) -> Self {
        Scraper {
            client,
            date_range,
            location_level: list_level,
            list_type,
            loc_df,
            time_range,
        }
    }

    /// Make vector of all locations consisting of all country, region, sub-region, and (if applicable)
    /// hotspot combinations.
    fn make_loc_vec(&self) -> Vec<LocationRow> {
        let loc_vec = if self.location_level == LocationLevel::Hotspot {
            HOTSPOT_COLUMNS
        } else {
            REGION_COLUMNS
        };
        let mut loc = match self.loc_df.columns(loc_vec) {
            Ok(l) => l,
            Err(e) => panic!("{}", e),
        }
        .iter()
        .map(|&s| s.iter())
        .collect::<Vec<_>>();
        (0..self.loc_df.shape().0)
            .map(|_| LocationRow::new(&mut loc))
            .collect::<Vec<LocationRow>>()
    }

    fn make_loc_payload(&self) -> Vec<Vec<(String, String)>> {
        let location_level_code = self.location_level.to_string();
        let columns = if self.list_type == ListType::Global {
            vec![location_level_code]
        } else {
            let list_type_code = self.list_type.to_string();
            vec![location_level_code, list_type_code]
        };
        let mut col_iters = match self.loc_df.columns(columns) {
            Ok(loc_columns) => loc_columns,
            Err(e) => panic!("{}", e),
        }
        .iter()
        .map(|&s| s.iter())
        .collect::<Vec<_>>();
        let mut loc_payload = (0..self.loc_df.shape().0)
            .map(|_| {
                let mut payload = vec![];
                let mut r = 1;
                for iter in &mut col_iters {
                    let value = iter.next().unwrap().to_string();
                    payload.push((format!("r{r}"), remove_quote(&value)));
                    r += 1;
                }
                payload
            })
            .collect::<Vec<Vec<(String, String)>>>();

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
            .collect::<Vec<Vec<(String, u8)>>>()
    }

    pub(super) fn get_response(
        &self,
        loc: &Vec<(String, String)>,
        time: &Vec<(String, u8)>,
        date_query: &Vec<(&str, String)>,
        sleep: u64,
    ) -> Response {
        match self
            .client
            .get(BASE_URL)
            .query(loc)
            .query(time)
            .query(date_query.deref())
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

    pub fn scrape_pages(self) -> DataFrame {
        let date_query = Arc::new(vec![("t2", self.date_range.to_string())]);
        let selectors = Arc::new(Selectors::new());
        let (doc_selector, doc_format) = if self.location_level == LocationLevel::Hotspot {
            (selectors.hotspot_select(), HOTSPOT)
        } else {
            (selectors.region_select(), REGION)
        };
        let loc_query = self.make_loc_payload();
        let loc_vec = self.make_loc_vec();
        let time_query = self.make_time_payload();
        let arc_scraper = Arc::new(self);
        let loc_payload = loc_vec
            .into_iter()
            .zip(loc_query)
            .collect::<Vec<(LocationRow, Vec<(String, String)>)>>();
        let payloads = loc_payload
            .into_iter()
            .cartesian_product(time_query)
            .collect::<Vec<_>>();
        let s = Instant::now();
        let style =
            ProgressStyle::with_template("{bar:100} {pos:>7}/{len:7} [{elapsed}] [{eta}]").unwrap();
        let output_list = payloads
            .into_par_iter()
            .progress_with_style(style)
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
}
