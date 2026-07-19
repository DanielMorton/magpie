use crate::error::Result;
use crate::login;
use crate::target::{Scraper, SpeciesArgs};

pub fn run(args: SpeciesArgs) -> Result<()> {
    let (loc_file, list_level) = args.get_loc_data()?;
    let list_type = args.get_list_type(&args.location_options)?;
    let date_range = args.get_date_range()?;
    let time_range = args.get_time_range()?;
    let output_file = args.get_output_file()?;

    let client = login::login()?;

    let scraper = Scraper::new(
        client, date_range, list_level, list_type, loc_file, time_range,
    )?;

    scraper.scrape_all(output_file)?;

    Ok(())
}
