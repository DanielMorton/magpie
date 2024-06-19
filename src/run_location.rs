use std::fs::File;
use crate::location::regions::{get_countries, get_regions, get_sub_regions};
use crate::location::selectors::Selectors;
use polars::prelude::{CsvWriter, NamedFrom, Series, SerWriter};
use rayon::prelude::*;
use std::time::Instant;
use polars::functions::concat_df_diagonal;

use crate::location::df::{country_to_df, filter_join_df, region_to_df, sub_region_to_df};
use reqwest::blocking::Client;

pub fn print_hms(start: &Instant) {
    let millis = start.elapsed().as_millis();
    let seconds = millis / 1000;
    let (hour, minute, second) = (seconds / 3600, (seconds % 3600) / 60, seconds % 60);
    println!("{:02}:{:02}:{:02}.{}", hour, minute, second, millis % 1000)
}

pub fn run() {
    let client = Client::builder().cookie_store(true).build().unwrap();
    let selectors = Selectors::new();

    let s = Instant::now();
    let countries = get_countries(&client, &selectors);
    let regions = countries
        .par_iter()
        .map(|c| get_regions(&client, &selectors, c))
        .flatten()
        .collect::<Vec<_>>();
    let sub_regions = regions
        .par_iter()
        .map(|c| get_sub_regions(&client, &selectors, c))
        .flatten()
        .collect::<Vec<_>>();
    let country_df = country_to_df(&countries);
    let region_df = region_to_df(&regions);
    let sub_region_df = sub_region_to_df(&sub_regions);
    let mut country_filter_df = filter_join_df(
        &country_df,
        &region_df,
        &["country", "country_code"],
        "region",
    );
    let mut region_filter_df = filter_join_df(
        &region_df,
        &sub_region_df,
        &["country", "country_code", "region", "region_code"],
        "sub_region",
    );
    match country_filter_df.with_column(Series::new(
        "region",
        country_filter_df.column("country").unwrap(),
    )) {
        Ok(_) => (),
        Err(e) => panic!("{:?}", e),
    };
    match country_filter_df.with_column(Series::new(
        "region_code",
        country_filter_df.column("country_code").unwrap(),
    )) {
        Ok(_) => (),
        Err(e) => panic!("{:?}", e),
    };
    match country_filter_df.with_column(Series::new(
        "sub_region",
        country_filter_df.column("country").unwrap(),
    )) {
        Ok(_) => (),
        Err(e) => panic!("{:?}", e),
    };
    match country_filter_df.with_column(Series::new(
        "sub_region_code",
        country_filter_df.column("country_code").unwrap(),
    )) {
        Ok(_) => (),
        Err(e) => panic!("{:?}", e),
    };
    match region_filter_df.with_column(Series::new(
        "sub_region",
        region_filter_df.column("region").unwrap(),
    )) {
        Ok(_) => (),
        Err(e) => panic!("{:?}", e),
    };
    match region_filter_df.with_column(Series::new(
        "sub_region_code",
        region_filter_df.column("region_code").unwrap(),
    )) {
        Ok(_) => (),
        Err(e) => panic!("{:?}", e),
    };
    let mut output = match concat_df_diagonal(&[country_filter_df, region_filter_df, sub_region_df]) {
        Ok(df) => df,
        Err(e) => panic!("{:?}", e)
    };
    let file = match File::create("regions_pl.csv") {
        Ok(f) => f,
        Err(e) => panic!("{}", e),
    };
    match CsvWriter::new(&file)
        .include_header(true)
        .finish(&mut output)
    {
        Ok(_) => (),
        Err(e) => panic!("{:?}", e),
    }
    print_hms(&s)
}
