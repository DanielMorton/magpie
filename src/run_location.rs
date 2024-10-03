use std::error::Error;
use std::fs::File;
use std::time::Instant;

use polars::prelude::{CsvWriter, SerWriter};
use rayon::prelude::*;
use reqwest::blocking::Client;

use crate::location::regions::{get_countries, get_regions, get_sub_regions};
use crate::location::df::{hotspot_to_df, sub_region_to_df};
use crate::location::hotspot::get_hotspots;
use crate::target::print_hms;

pub fn run() -> Result<(), Box<dyn Error>> {
    let client = Client::builder().cookie_store(true).build().unwrap();

    let mut s = Instant::now();
    let countries = get_countries(&client);
    let regions = countries
        .par_iter()
        .map(|c| get_regions(&client, c))
        .flatten()
        .collect::<Vec<_>>();
    let sub_regions = regions
        .par_iter()
        .map(|r| get_sub_regions(&client, r))
        .flatten()
        .collect::<Vec<_>>();
    let mut sub_region_df = sub_region_to_df(&sub_regions);
    println!("{}", countries.len());
    println!("{}", regions.len());
    println!("{}", sub_regions.len());
    println!("{:?}", sub_region_df.shape());
    print_hms(&s);
    s = Instant::now();
    let hotspots = sub_regions
        .par_iter()
        .map(|s| get_hotspots(&client, s))
        .flatten()
        .collect::<Vec<_>>();
    let mut hotspot_df = hotspot_to_df(&hotspots);
    println!("{:?}", hotspot_df.shape());
    print_hms(&s);
    let file = match File::create("regions_pl.csv") {
        Ok(f) => f,
        Err(e) => panic!("{}", e),
    };
    match CsvWriter::new(&file)
        .include_header(true)
        .finish(&mut sub_region_df)
    {
        Ok(_) => (),
        Err(e) => panic!("{:?}", e),
    }
    let hotspot_file = match File::create("hotspots_pl.csv") {
        Ok(f) => f,
        Err(e) => panic!("{}", e),
    };
    match CsvWriter::new(&hotspot_file)
        .include_header(true)
        .finish(&mut hotspot_df)
    {
        Ok(_) => (),
        Err(e) => panic!("{:?}", e),
    }
    Ok(())
}
