use crate::login::LOGIN_URL;
use crate::print_hms;
use crate::row::{LocationRow, SpeciesRow};
use crate::scrape_params::{DateRange, ListLevel, ListType};
use crate::selectors::Selectors;
use crate::table::{add_columns, empty_table};
use itertools::Itertools;
use polars::prelude::{DataFrame, Series};
use polars::prelude::NamedFrom;
use rayon::prelude::*;
use reqwest::blocking::{Client, Response};
use scraper::{ElementRef, Html};
use std::cmp::min;
use std::ops::Deref;
use std::str::FromStr;
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

static BASE_URL: &str = "https://ebird.org/targets";
static HOME_URL: &str = "https://ebird.org/home";

static MAX_BACKOFF: u64 = 256;

pub(crate) struct Scraper {
    client: Client,
    date_range: DateRange,
    list_level: ListLevel,
    list_type: ListType,
    loc_df: DataFrame,
    selectors: Selectors,
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
            selectors: Selectors::new(),
            time_range,
        }
    }

    fn make_loc_vec(&self) -> Vec<LocationRow> {
        let loc_vec = if self.list_level == ListLevel::Hotspot {
            vec!["country", "region", "sub_region", "hotspot"]
        } else {
            vec!["country", "region", "sub_region"]
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
                    l.push(iter.next().unwrap().to_string().replace("\"", ""))
                }
                LocationRow::new(l)
            })
            .collect::<Vec<LocationRow>>()
    }

    fn make_loc_payload(&self) -> Vec<Vec<(String, String)>> {
        let list_level_code = self.list_level.to_string() + "_code";
        let columns = if self.list_type == ListType::Global {
            vec![list_level_code]
        } else {
            let list_type_code = self.list_type.to_string() + "_code";
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
                    payload.push((
                        format!("r{r}"),
                        iter.next().unwrap().to_string().replace("\"", ""),
                    ));
                    r += 1;
                }
                payload
            })
            .collect::<Vec<Vec<(String, String)>>>();

        if self.list_type == ListType::Global {
            loc_payload.iter_mut().for_each(|payload| {
                payload.push((format!("r2"), format!("world")));
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

    fn get_response(
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
            Ok(r) => {
                let url = r.url().to_string();
                if url.contains(LOGIN_URL) || url.contains(HOME_URL) {
                    thread::sleep(Duration::from_secs(sleep));
                    self.get_response(loc, time, date_query, min(MAX_BACKOFF, 2 * sleep))
                } else {
                    r
                }
            }
            Err(_) => {
                thread::sleep(Duration::from_secs(sleep));
                self.get_response(loc, time, date_query, min(MAX_BACKOFF, 2 * sleep))
            }
        }
    }

    fn scrape_page(
        &self,
        loc: Vec<(String, String)>,
        time: &Vec<(String, u8)>,
        date_query: &Vec<(&str, String)>,
        sleep: u64,
    ) -> DataFrame {
        let loc_code = &loc[0].1;
        let response = self.get_response(&loc, &time, date_query, sleep);
        //let url = response.url().to_string();
        let doc = match response.text() {
            Ok(text) => Html::parse_document(&text),
            Err(e) => {
                println!("{}", e);
                //println!("HTML Empty {} {} {}", url, loc_code, &sleep);
                thread::sleep(Duration::from_secs(sleep));
                return self.scrape_page(loc, time, date_query, min(MAX_BACKOFF, 2 * sleep));
            }
        };
        let (doc_selector, doc_format) = if self.list_level == ListLevel::Hotspot {
            (self.selectors.hotspot_select(), "hotspot")
        } else {
            (self.selectors.region_select(), "region")
        };
        match doc
            .select(doc_selector)
            .next()
            .map(|r| r.value().attr("href").unwrap())
            .filter(|&r| r == format!("{}/{}", doc_format, loc_code))
        {
            Some(_) => (),
            None => {
                //println!("Hotspot Empty {} {} {}", url, loc_code, &sleep);
                thread::sleep(Duration::from_secs(sleep));
                return self.scrape_page(loc, time, date_query, min(MAX_BACKOFF, 2 * sleep));
            }
        }
        match doc
            .select(&self.selectors.species_count())
            .next()
            .map(|count| count.text().next())
            .flatten()
            .map(|count| u32::from_str(count).ok())
            .flatten()
        {
            Some(0) => empty_table(),
            Some(_) => match doc.select(&self.selectors.native()).next() {
                Some(t) => self.scrape_table(t),
                None => empty_table(),
            },
            None => {
                //println!("Doc Empty {} {} {}", url, loc_code, &sleep);
                thread::sleep(Duration::from_secs(sleep));
                self.scrape_page(loc, time, date_query, min(MAX_BACKOFF, 2 * sleep))
            }
        }
    }

    fn scrape_table(&self, table: ElementRef) -> DataFrame {
        let df_row = table
            .select(&self.selectors.rows())
            .map(|row| {
                let species = row
                    .select(&self.selectors.species())
                    .next()
                    .map(|s| s.select(&self.selectors.a()).next())
                    .flatten();
                let common_name = species
                    .map(|s| s.text().next())
                    .flatten()
                    .unwrap_or("")
                    .trim();
                let scientific_name = species
                    .map(|s| {
                        s.select(&self.selectors.sci_name())
                            .next()
                            .map(|s| s.text().next())
                            .flatten()
                    })
                    .flatten()
                    .unwrap_or("")
                    .trim();
                let percent = row
                    .select(&self.selectors.percent())
                    .next()
                    .map(|p| p.value().attr("title"))
                    .flatten()
                    .map(|p| p.split("%").next())
                    .flatten()
                    .map(|p| match f32::from_str(p) {
                        Ok(f) => f,
                        Err(e) => panic!("{}", e),
                    })
                    .unwrap_or(0.0);
                SpeciesRow::new(common_name, scientific_name, percent)
            })
            .collect::<Vec<_>>();
        let common_name = Series::new(
            "common name",
            df_row.iter().map(|r| r.common_name()).collect::<Vec<_>>(),
        );
        let scietific_name = Series::new(
            "scientific name",
            df_row
                .iter()
                .map(|r| r.scientific_name())
                .collect::<Vec<_>>(),
        );
        let percent = Series::new(
            "percent",
            df_row.iter().map(|r| r.percent()).collect::<Vec<_>>(),
        );
        return DataFrame::new(vec![common_name, scietific_name, percent]).unwrap();
    }
}

pub(crate) fn scrape_pages(scraper: Scraper) -> DataFrame {
    let date_query = Arc::new(vec![("t2", scraper.date_range.to_string())]);
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
            let mut df = arc_scraper.scrape_page(loc, &time, &date_query, 1);
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
