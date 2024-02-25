mod loc;
mod login;
mod parse;
mod scraper;

extern crate strum;
#[macro_use]
extern crate strum_macros;

use crate::loc::load_data;
use parse::MagpieParse;
use polars::io::prelude::*;
use polars::prelude::CsvWriter;
use scraper::Scraper;
use std::fs::File;

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
    let mut output = scraper.scrape_pages();
    let file = match File::create(output_file) {
        Ok(f) => f,
        Err(e) => panic!("{}", e),
    };
    match CsvWriter::new(&file).include_header(true).finish(&mut output) {
        Ok(_) => (),
        Err(e) => panic!("{}", e),
    }
}
