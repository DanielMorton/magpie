use crate::error::Result;
use crate::login;
use crate::target::{Scraper, SpeciesArgs};
use crate::utils::load_csv;
use polars::prelude::{CsvWriter, SerWriter};
use std::fs::File;

pub fn run(args: SpeciesArgs) -> Result<()> {
    let (loc_file, list_level) = args.get_loc_data()?;
    let loc_df = load_csv(loc_file)?;
    let list_type = args.get_list_type(&args.location_options)?;
    let date_range = args.get_date_range()?;
    let time_range = args.get_time_range()?;
    let output_file = args.get_output_file()?;

    let client = login::login()?;

    let scraper = Scraper::new(
        client, date_range, list_level, list_type, loc_df, time_range,
    );

    let mut output = scraper.scrape_all()?;

    let file = File::create(output_file)?;
    CsvWriter::new(&file)
        .include_header(true)
        .finish(&mut output)?;

    Ok(())
}
