mod login;
mod parse;
mod row;
mod scrape_params;
mod scraper;
mod selectors;

extern crate strum;
#[macro_use]
extern crate strum_macros;

use parse::MagpieParse;
use polars::frame::DataFrame;
use polars::io::prelude::*;
use polars::prelude::{CsvWriter, LazyCsvReader, LazyFileListReader};
use scraper::{scrape_pages,Scraper};
use std::fs::File;
use std::time::Instant;

fn load_data(loc_file: &str) -> DataFrame {
    match LazyCsvReader::new(loc_file)
        .has_header(true)
        .finish()
        .map(|f| f.collect())
    {
        Ok(r) => match r {
            Ok(region) => region,
            Err(e) => panic!("Failed to load {}:\n {}", loc_file, e),
        },
        Err(e) => panic!("Failed to load {}:\n {}", loc_file, e),
    }
}

pub fn print_hms(start: &Instant) {
    let millis = start.elapsed().as_millis();
    let seconds = millis / 1000;
    let (hour, minute, second) = (seconds / 3600, (seconds % 3600) / 60, seconds % 60);
    println!("{:02}:{:02}:{:02}.{}", hour, minute, second, millis % 1000)
}

fn main() {
    let matches = parse::parse();

    let (loc_file, list_level) = matches.get_loc_data();
    let loc_df = load_data(loc_file);
    let list_type = matches.get_list_type();
    let date_range = matches.get_date_range();
    let time_range = matches.get_time_range();
    let output_file = matches.get_output_file();

    let client = login::login();

    let scraper = Scraper::new(
        client, date_range, list_level, list_type, loc_df, time_range,
    );
    let mut output = scrape_pages(scraper);
    let file = match File::create(output_file) {
        Ok(f) => f,
        Err(e) => panic!("{}", e),
    };
    match CsvWriter::new(&file).has_header(true).finish(&mut output) {
        Ok(_) => (),
        Err(e) => panic!("{}", e),
    }
}
