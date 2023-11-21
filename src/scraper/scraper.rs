use crate::scraper::row::LocationRow;
use crate::scraper::scrape_params::{DateRange, ListLevel, ListType};
use crate::scraper::{BASE_URL, HOME_URL, HOTSPOT_COLUMNS, LOGIN_URL, REGION_COLUMNS};
use polars::prelude::DataFrame;
use reqwest::blocking::{Client, Response};
use std::ops::Deref;
use std::thread;
use std::time::Duration;

pub struct Scraper {
    client: Client,
    pub(super) date_range: DateRange,
    pub(super) list_level: ListLevel,
    list_type: ListType,
    loc_df: DataFrame,
    time_range: Vec<(u8, u8)>,
}

impl Scraper {
    pub(crate) fn new(
        client: Client,
        date_range: DateRange,
        list_level: ListLevel,
        list_type: ListType,
        loc_df: DataFrame,
        time_range: Vec<(u8, u8)>,
    ) -> Self {
        Scraper {
            client,
            date_range,
            list_level,
            list_type,
            loc_df,
            time_range,
        }
    }

    fn remove_quote(&self, value: &str) -> String {
        let mut chars = value.chars();
        chars.next();
        chars.next_back();
        chars.as_str().to_string()
    }

    pub(super) fn make_loc_vec(&self) -> Vec<LocationRow> {
        let loc_vec = if self.list_level == ListLevel::Hotspot {
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
            .map(|_| {
                let mut l = Vec::new();
                for iter in &mut loc {
                    let value = iter.next().unwrap().to_string();
                    l.push(self.remove_quote(&value))
                }
                LocationRow::new(l)
            })
            .collect::<Vec<LocationRow>>()
    }

    pub(super) fn make_loc_payload(&self) -> Vec<Vec<(String, String)>> {
        let list_level_code = self.list_level.to_code();
        let columns = if self.list_type == ListType::Global {
            vec![list_level_code]
        } else {
            let list_type_code = self.list_type.to_code();
            vec![list_level_code, list_type_code]
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
                    payload.push((format!("r{r}"), self.remove_quote(&value)));
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

    pub(super) fn make_time_payload(&self) -> Vec<Vec<(String, u8)>> {
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
}
