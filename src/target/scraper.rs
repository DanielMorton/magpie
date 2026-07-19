use crate::error::{AppError, Result};
use crate::selectors;
use crate::target::row::LocationRow;
use crate::target::scrape_params::{DateRange, ListType, LocationLevel};
use crate::target::scrape_table::SpeciesRecord;
use crate::utils::print_elapsed;
use indicatif::{MultiProgress, ParallelProgressIterator, ProgressBar, ProgressStyle};
use itertools::Itertools;
use rayon::prelude::*;
use reqwest::blocking::Client;
use scraper::Html;
use std::cmp::min;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};
use tracing::{error, info, warn};

const BASE_URL: &str = "https://ebird.org/targets";
const HOME_URL: &str = "https://ebird.org/home";
const LOGIN_URL: &str = "https://secure.birds.cornell.edu/cassso/login";
const MAX_BACKOFF_SECS: u64 = 100;
const MIN_BACKOFF_SECS: u64 = 5;
const MAX_RETRIES: usize = 10;

#[derive(Debug, Clone)]
struct ScrapeFailure {
    loc_code: String,
    url: String,
    reason: String,
    time: (u8, u8),
}

#[derive(Debug, Clone)]
struct LocationData {
    pub row: LocationRow,
    pub codes: Vec<(String, String)>,
}

#[derive(Debug, Clone)]
pub struct EnrichedRecord {
    pub common_name: String,
    pub scientific_name: String,
    pub percent: f32,
    pub checklists: i32,
    pub sub_region: String,
    pub region: String,
    pub country: String,
    pub hotspot: Option<String>,
    pub start_month: u32,
    pub end_month: u32,
}

impl EnrichedRecord {
    fn from(record: SpeciesRecord, row: &LocationRow, time: &[(String, u8)]) -> Self {
        Self {
            common_name: record.common_name,
            scientific_name: record.scientific_name,
            percent: record.percent,
            checklists: record.checklists,
            sub_region: row.sub_region.clone(),
            region: row.region.clone(),
            country: row.country.clone(),
            hotspot: row.hotspot.clone(),
            start_month: time[0].1 as u32,
            end_month: time[1].1 as u32,
        }
    }
}

pub struct Scraper {
    client: Client,
    date_range: DateRange,
    location_level: LocationLevel,
    list_type: ListType,
    locations: Vec<LocationData>,
    time_ranges: Vec<(u8, u8)>,
}

impl Scraper {
    pub fn new(
        client: Client,
        date_range: DateRange,
        location_level: LocationLevel,
        list_type: ListType,
        loc_file: &str,
        time_ranges: Vec<(u8, u8)>,
    ) -> Result<Self> {
        let locations = Self::load_locations(loc_file, location_level, list_type)?;
        Ok(Self {
            client,
            date_range,
            location_level,
            list_type,
            locations,
            time_ranges,
        })
    }

    fn load_locations(
        path: &str,
        level: LocationLevel,
        list_type: ListType,
    ) -> Result<Vec<LocationData>> {
        let mut reader = csv::Reader::from_path(path)?;
        let headers: Vec<String> = reader.headers()?.iter().map(|s| s.to_string()).collect();

        let has_hotspot = level == LocationLevel::Hotspot;
        let level_code_col = match level {
            LocationLevel::Hotspot => "hotspot_code",
            LocationLevel::SubRegion => "sub_region_code",
        };

        let second_code_col = format!("{}_code", list_type);
        let need_cols: Vec<&str> = if list_type == ListType::Global {
            vec![level_code_col]
        } else {
            vec![level_code_col, &second_code_col]
        };

        let mut col_indices = Vec::new();
        for col in &need_cols {
            let idx = headers.iter().position(|h| h == *col).ok_or_else(|| {
                AppError::Parse(format!("Column '{}' not found in CSV", col))
            })?;
            col_indices.push(idx);
        }

        let mut locations = Vec::new();
        for result in reader.records() {
            let record = result?;
            let row = LocationRow::from_csv_record(&record, has_hotspot)?;

            let mut codes = Vec::new();
            for (i, &col_idx) in col_indices.iter().enumerate() {
                let code = record
                    .get(col_idx)
                    .ok_or_else(|| AppError::Parse("Missing code value".into()))?
                    .trim_matches('"')
                    .to_string();
                codes.push((format!("r{}", i + 1), code));
            }

            if list_type == ListType::Global {
                codes.push(("r2".into(), "world".into()));
            }

            locations.push(LocationData { row, codes });
        }

        Ok(locations)
    }

    fn fetch_with_backoff(
        &self,
        loc: &[(String, String)],
        time: &[(String, u8)],
        date_query: &[(&str, String)],
        sleep_secs: u64,
    ) -> Result<reqwest::blocking::Response> {
        let response = self
            .client
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
            self.fetch_with_backoff(
                loc,
                time,
                date_query,
                min(sleep_secs * 2, MAX_BACKOFF_SECS),
            )
        } else {
            Ok(response)
        }
    }

    fn scrape_single_attempt(
        &self,
        loc: &[(String, String)],
        time: &[(String, u8)],
    ) -> Result<(Vec<SpeciesRecord>, bool)> {
        let loc_code = loc[0].1.clone();
        let date_query = vec![("t2", self.date_range.to_string())];

        let response = self.fetch_with_backoff(loc, time, &date_query, MIN_BACKOFF_SECS)?;
        let text = response.text()?;
        let doc = Html::parse_document(&text);

        let (selector, format) = match self.location_level {
            LocationLevel::Hotspot => (selectors::target::hotspot_select(), "hotspot"),
            LocationLevel::SubRegion => (selectors::target::region_select(), "region"),
        };

        let valid = doc
            .select(selector)
            .next()
            .and_then(|r| r.value().attr("href"))
            .map(|href| href == format!("{}/{}", format, loc_code))
            .unwrap_or(false);

        if !valid {
            return Ok((Vec::new(), false));
        }

        let checklists = doc
            .select(selectors::target::checklists())
            .next()
            .and_then(|el| el.text().next())
            .and_then(|text| {
                text.chars()
                    .filter(|c| c.is_numeric())
                    .collect::<String>()
                    .parse()
                    .ok()
            })
            .unwrap_or(0);

        let species_count = doc
            .select(selectors::target::species_count())
            .next()
            .and_then(|c| c.text().next())
            .and_then(|c| c.parse::<u32>().ok());

        match species_count {
            Some(0) | None => Ok((Vec::new(), true)),
            Some(_) => {
                let records = doc
                    .select(selectors::target::native())
                    .next()
                    .map_or_else(|| Ok(Vec::new()), |t| {
                        crate::target::scrape_table::scrape_table(t, checklists)
                    })?;
                Ok((records, true))
            }
        }
    }

    fn scrape_single(
        &self,
        loc_data: &LocationData,
        time: Vec<(String, u8)>,
    ) -> Result<(Vec<EnrichedRecord>, Option<ScrapeFailure>)> {
        let loc_code = loc_data.codes[0].1.clone();
        let time_tuple = (time[0].1, time[1].1);
        let mut last_err = None;

        for attempt in 0..MAX_RETRIES {
            if attempt > 0 {
                let backoff = min(2u64.pow(attempt as u32), 30);
                thread::sleep(Duration::from_secs(backoff));
                info!(
                    "Retrying {} (attempt {}/{})",
                    loc_code,
                    attempt + 1,
                    MAX_RETRIES
                );
            }

            match self.scrape_single_attempt(&loc_data.codes, &time) {
                Ok((records, true)) => {
                    let enriched: Vec<EnrichedRecord> = records
                        .into_iter()
                        .map(|r| EnrichedRecord::from(r, &loc_data.row, &time))
                        .collect();
                    return Ok((enriched, None));
                }
                Ok((_, false)) => {
                    last_err = Some("Page validation failed".to_string());
                }
                Err(e) => {
                    last_err = Some(e.to_string());
                }
            }
        }

        let failure = ScrapeFailure {
            loc_code: loc_code.clone(),
            url: format!("{}?{}", BASE_URL, self.build_query_string(&loc_data.codes, &time)),
            reason: last_err.unwrap_or_else(|| "Unknown error".to_string()),
            time: time_tuple,
        };

        warn!(
            "Failed after {} retries for {}: {}",
            MAX_RETRIES, loc_code, failure.reason
        );
        Ok((Vec::new(), Some(failure)))
    }

    fn build_query_string(&self, loc: &[(String, String)], time: &[(String, u8)]) -> String {
        let mut parts: Vec<String> =
            loc.iter().map(|(k, v)| format!("{}={}", k, v)).collect();
        parts.push(format!("bmo={}&emo={}", time[0].1, time[1].1));
        parts.push(format!("t2={}", self.date_range));
        parts.join("&")
    }

    pub fn scrape_all(&self, output_file: &str) -> Result<()> {
        let start = Instant::now();

        let items: Vec<_> = self
            .locations
            .iter()
            .cartesian_product(
                self.time_ranges
                    .clone()
                    .into_iter()
                    .map(|(s, e)| vec![("bmo".into(), s), ("emo".into(), e)]),
            )
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

        let failures = Arc::new(std::sync::Mutex::new(Vec::new()));
        let failure_count = Arc::new(AtomicUsize::new(0));

        let all_records: Vec<Vec<EnrichedRecord>> = items
            .into_par_iter()
            .progress_with(pb.clone())
            .filter_map(|(loc_data, time)| {
                match self.scrape_single(loc_data, time) {
                    Ok((records, None)) => Some(records),
                    Ok((_, Some(failure))) => {
                        let mut f = failures.lock().unwrap();
                        f.push(failure);
                        failure_count.fetch_add(1, Ordering::Relaxed);
                        None
                    }
                    Err(e) => {
                        error!("Scrape failed for {}: {}", loc_data.row.country, e);
                        None
                    }
                }
            })
            .collect();

        pb.finish_with_message("Done");
        print_elapsed(&start, "Scraping complete");

        let failures = Arc::try_unwrap(failures).unwrap().into_inner().unwrap();
        let failure_count = failure_count.load(Ordering::Relaxed);

        if !failures.is_empty() {
            self.write_failures(&failures)?;
            warn!(
                "{} scrapes failed. See failures.csv for details.",
                failure_count
            );
        }

        let mut writer = csv::Writer::from_path(output_file)?;
        writer.write_record(&[
            "common name",
            "scientific name",
            "percent",
            "checklists",
            "sub_region",
            "region",
            "country",
            "hotspot",
            "start month",
            "end month",
        ])?;

        let mut total_records = 0usize;
        for batch in all_records {
            for record in batch {
                writer.write_record(&[
                    &record.common_name,
                    &record.scientific_name,
                    &record.percent.to_string(),
                    &record.checklists.to_string(),
                    &record.sub_region,
                    &record.region,
                    &record.country,
                    record.hotspot.as_deref().unwrap_or(""),
                    &record.start_month.to_string(),
                    &record.end_month.to_string(),
                ])?;
                total_records += 1;
            }
        }

        writer.flush()?;
        info!("Wrote {} records to {}", total_records, output_file);

        if failure_count > 0 && total_records == 0 {
            return Err(AppError::ScrapingFailures(failure_count));
        }

        Ok(())
    }

    fn write_failures(&self, failures: &[ScrapeFailure]) -> Result<()> {
        use std::io::Write;

        let mut file = std::fs::File::create("failures.csv")?;
        writeln!(file, "loc_code,url,reason,start_month,end_month")?;
        for f in failures {
            writeln!(
                file,
                "{},{},{},{},{}",
                f.loc_code,
                f.url.replace(',', "%2C"),
                f.reason.replace(',', ";"),
                f.time.0,
                f.time.1
            )?;
        }
        Ok(())
    }
}