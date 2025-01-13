use std::error::Error;
use std::fs::File;

use polars::prelude::*;

use crate::loc::load_data;
use crate::login;
use crate::target::{Scraper, SpeciesArgs};

pub(crate) fn run(args: SpeciesArgs) -> Result<(), Box<dyn Error>> {
    let (loc_file, list_level) = args.get_loc_data()?;
    let loc_df = load_data(loc_file);
    let list_type = args.get_list_type()?;
    let date_range = args.get_date_range()?;
    let time_range = args.get_time_range()?;
    let output_file = args.get_output_file()?;

    let client = login::login()?;

    let scraper = Scraper::new(
        client, date_range, list_level, list_type, loc_df, time_range,
    );

    let mut output = scraper.scrape_pages()?;

    let file = File::create(output_file)?;
    CsvWriter::new(&file)
        .include_header(true)
        .finish(&mut output)?;

    Ok(())
}
