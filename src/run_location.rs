use crate::location::regions::{get_countries, get_regions, get_sub_regions};
use crate::location::selectors::Selectors;
use polars::prelude::{CsvWriter, SerWriter};
use rayon::prelude::*;
use std::fs::File;
use std::time::Instant;

use crate::location::df::{hotspot_to_df, sub_region_to_df};
use reqwest::blocking::Client;
use crate::location::hotspot::get_hotspots;

pub fn print_hms(start: &Instant) {
    let millis = start.elapsed().as_millis();
    let seconds = millis / 1000;
    let (hour, minute, second) = (seconds / 3600, (seconds % 3600) / 60, seconds % 60);
    println!("{:02}:{:02}:{:02}.{}", hour, minute, second, millis % 1000)
}

pub fn run() {
    let client = Client::builder().cookie_store(true).build().unwrap();
    let selectors = Selectors::new();

    let mut s = Instant::now();
    let countries = get_countries(&client, &selectors);
    let regions = countries
        .par_iter()
        .map(|c| get_regions(&client, &selectors, c))
        .flatten()
        .collect::<Vec<_>>();
    let sub_regions = regions
        .par_iter()
        .map(|r| get_sub_regions(&client, &selectors, r))
        .flatten()
        .collect::<Vec<_>>();
    let mut sub_region_df = sub_region_to_df(&sub_regions);
    println!("{}", sub_region_df);
    print_hms(&s);
    s = Instant::now();
    let hotspots = sub_regions.par_iter()
        .map(|s| get_hotspots(&client, &selectors, s))
        .flatten()
        .collect::<Vec<_>>();
    let mut hotspot_df = hotspot_to_df(&hotspots);
    println!("{}", hotspot_df);
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
}
