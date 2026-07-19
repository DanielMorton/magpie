use crate::error::Result;
use crate::location::df::{hotspots_to_df, sub_regions_to_df};
use crate::location::regions::{get_countries, get_hotspots, get_regions, get_sub_regions};
use crate::utils::{print_elapsed, write_csv};
use indicatif::{MultiProgress, ParallelProgressIterator, ProgressBar, ProgressStyle};
use rayon::prelude::*;
use reqwest::blocking::Client;
use std::time::Instant;
use tracing::info;

pub fn run() -> Result<()> {
    let client = Client::builder().cookie_store(true).build()?;
    let start = Instant::now();
    let mp = MultiProgress::new();
    let style = ProgressStyle::with_template(
        "{spinner:.green} [{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
    )
    .unwrap()
    .progress_chars("##-");

    info!("Fetching countries...");
    let countries = get_countries(&client)?;
    info!("Found {} countries", countries.len());

    let pb = mp.add(ProgressBar::new(countries.len() as u64));
    pb.set_style(style.clone());
    pb.set_message("Regions");

    let regions: Vec<_> = countries
        .par_iter()
        .progress_with(pb.clone())
        .flat_map(|c| get_regions(&client, c))
        .collect();
    pb.finish_with_message("Regions done");
    info!("Found {} regions", regions.len());

    let pb = mp.add(ProgressBar::new(regions.len() as u64));
    pb.set_style(style.clone());
    pb.set_message("Sub-regions");

    let sub_regions: Vec<_> = regions
        .par_iter()
        .progress_with(pb.clone())
        .flat_map(|r| get_sub_regions(&client, r))
        .collect();
    pb.finish_with_message("Sub-regions done");
    info!("Found {} sub-regions", sub_regions.len());

    let mut sub_region_df = sub_regions_to_df(&sub_regions)?;
    write_csv(&mut sub_region_df, "regions.csv")?;
    print_elapsed(&start, "Sub-regions scraped");

    let hotspot_start = Instant::now();
    let pb = mp.add(ProgressBar::new(sub_regions.len() as u64));
    pb.set_style(style.clone());
    pb.set_message("Hotspots");

    let hotspots: Vec<_> = sub_regions
        .par_iter()
        .progress_with(pb.clone())
        .flat_map(|s| get_hotspots(&client, s))
        .collect();
    pb.finish_with_message("Hotspots done");
    info!("Found {} hotspots", hotspots.len());

    let mut hotspot_df = hotspots_to_df(&hotspots)?;
    write_csv(&mut hotspot_df, "hotspots.csv")?;
    print_elapsed(&hotspot_start, "Hotspots scraped");
    print_elapsed(&start, "Total time");

    Ok(())
}
