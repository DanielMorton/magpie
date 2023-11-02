use std::ops::Deref;
use crate::print_hms;
use crate::row::{LocationRow, SpeciesRow};
use crate::scrape_params::{DateRange, ListLevel, ListType};
use crate::selectors::Selectors;
use itertools::Itertools;
use polars::frame::DataFrame;
use polars::prelude::NamedFrom;
use polars::series::Series;
use reqwest::blocking::{Client, Response};
use scraper::{ElementRef, Html};
use std::str::FromStr;
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};
use crate::login::LOGIN_URL;

static BASE_URL: &str = "https://ebird.org/targets";

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

    fn add_columns(&self, df: &mut DataFrame, row: LocationRow, time: &Vec<(String, u8)>) {
        let size = df.shape().0;
        df.with_column(Series::new("sub_region", vec![row.sub_region(); size]))
            .unwrap();
        df.with_column(Series::new("region", vec![row.region(); size]))
            .unwrap();
        df.with_column(Series::new("country", vec![row.country(); size]))
            .unwrap();
        row.hotspot().iter().for_each(|hotspot| {
            df.with_column(Series::new("hotspot", vec![hotspot.clone(); size]))
                .unwrap();
        });
        df.with_column(Series::new("start month", vec![time[0].1 as u32; size]))
            .unwrap();
        df.with_column(Series::new("end month", vec![time[1].1 as u32; size]))
            .unwrap();
    }

    fn empty_table(&self) -> DataFrame {
        let common_name = Series::new("common name", Vec::<String>::new());
        let scietific_name = Series::new("scientific name", Vec::<String>::new());
        let percent = Series::new("percent", Vec::<f32>::new());
        return DataFrame::new(vec![common_name, scietific_name, percent]).unwrap();
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
        date_query: &Arc<Vec<(&str, String)>>,
        sleep: u64,
    ) -> Response {
        match self
            .client
            .get(BASE_URL)
            .query(loc)
            .query(time)
            .query(date_query.deref())
            .send() {
            Ok(r) => {
                if r.url().to_string().contains(LOGIN_URL) {
                    thread::sleep(Duration::from_secs(sleep));
                    self.get_response(loc, time, date_query, 2 * sleep)
                } else {
                    r
                }
            }
            Err(_) => {
                thread::sleep(Duration::from_secs(sleep));
                self.get_response(loc, time, date_query, 2 * sleep)
            }
        }
    }

    fn scrape_page(
        &self,
        loc: Vec<(String, String)>,
        time: &Arc<Vec<(String, u8)>>,
        date_query: Arc<Vec<(&str, String)>>,
        sleep: u64,
    ) -> DataFrame {
        let response = self.get_response(&loc, &time, &date_query, sleep);
        thread::sleep(Duration::from_secs(sleep));
        let doc = match response.text() {
            Ok(text) => Html::parse_document(&text),
            Err(e) => {
                println!("{}", e);
                return self.scrape_page(loc, time, date_query, 2 * sleep)
            },
        };
        match doc
            .select(&self.selectors.species_count())
            .next()
            .map(|count| count.text().next())
            .flatten()
            .map(|count| u32::from_str(count).ok())
            .flatten()
        {
            Some(0) => self.empty_table(),
            Some(_) => match doc.select(&self.selectors.native()).next() {
                Some(t) => self.scrape_table(t),
                None => {
                    thread::sleep(Duration::from_secs(sleep));
                    self.scrape_page(loc, time, date_query, 2 * sleep)
                }
            },
            None => {
                thread::sleep(Duration::from_secs(sleep));
                self.scrape_page(loc, time, date_query, 2 * sleep)
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
                SpeciesRow::new(
                    common_name,
                    scientific_name,
                    percent,
                )
            })
            .collect::<Vec<_>>();
        let common_name = Series::new(
            "common name",
            df_row.iter().map(|r| r.common_name()).collect::<Vec<_>>(),
        );
        let scietific_name = Series::new(
            "scientific name",
            df_row.iter().map(|r| r.scientific_name()).collect::<Vec<_>>(),
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
    let payloads = loc_payload.into_iter()
        .cartesian_product(time_query);
    let s = Instant::now();
    let mut threads = Vec::new();
    payloads.for_each(|((row, loc), time)| {

            let time_clone = Arc::new(time);
            let scraper_clone = arc_scraper.clone();
            let date_clone = date_query.clone();
            threads.push(
            thread::spawn(move ||{
                let time = time_clone;

                let mut df = scraper_clone.scrape_page(loc.clone(), &time, date_clone, 1);
                scraper_clone.add_columns(&mut df, row.clone(), &time);
                df
            }))
        });
    let output_list = threads.into_iter()
        .map(|t| t.join().unwrap()).collect::<Vec<_>>();

    print_hms(&s);
    output_list
        .into_iter()
        .reduce(|a, b| a.vstack(&b).unwrap())
        .unwrap()
}
