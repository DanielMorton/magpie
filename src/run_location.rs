use std::error::Error;
use std::fs::File;
use std::time::Instant;

use indicatif::ParallelProgressIterator;
use polars::prelude::{CsvWriter, SerWriter, DataFrame};
use rayon::prelude::*;
use reqwest::blocking::Client;

use crate::location::df::{hotspot_to_df, sub_region_to_df};
use crate::location::hotspot::get_hotspots;
use crate::location::regions::{get_countries, get_regions, get_sub_regions};
use crate::target::print_hms;

fn write_csv(df: &mut DataFrame, filename: &str) -> Result<(), Box<dyn Error>> {
    let file = File::create(filename)?;
    CsvWriter::new(&file)
        .include_header(true)
        .finish(df)
        .map_err(|e| e.into())
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let client = Client::builder().cookie_store(true).build()?;

    let start = Instant::now();
    let countries = get_countries(&client)?;

    let regions: Vec<_> = countries
        .par_iter()
        .progress()
        .flat_map(|c| get_regions(&client, c, 1))
        .collect();
    println!("Number of regions: {}", regions.len());

    let sub_regions: Vec<_> = regions
        .par_iter()
        .progress()
        .flat_map(|r| get_sub_regions(&client, r, 1))
        .collect();
    println!("Number of sub-regions: {}", sub_regions.len());

    let mut sub_region_df = sub_region_to_df(&sub_regions)?;
    print_hms(&start);

    let hotspot_start = Instant::now();
    let hotspots: Vec<_> = sub_regions
        .par_iter()
        .progress()
        .flat_map(|s| get_hotspots(&client, s, 1))
        .collect();
    println!("Number of hotspots: {}", hotspots.len());
    let mut hotspot_df = hotspot_to_df(&hotspots)?;
    print_hms(&hotspot_start);

    write_csv(&mut sub_region_df, "regions_pl.csv")?;
    write_csv(&mut hotspot_df, "hotspots_pl.csv")?;
    Ok(())
}