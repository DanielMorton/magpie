use crate::error::Result;
use crate::location::df::{hotspots_to_df, sub_regions_to_df};
use crate::location::regions::{get_countries, get_hotspots, get_regions, get_sub_regions};
use crate::utils::{print_elapsed, write_csv};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use reqwest::Client;
use std::time::Instant;
use tokio::task;
use tracing::info;

pub async fn run() -> Result<()> {
    let client = Client::builder().cookie_store(true).build()?;
    let start = Instant::now();
    let mp = MultiProgress::new();
    let style = ProgressStyle::with_template(
        "{spinner:.green} [{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
    ).unwrap().progress_chars("##-");

    info!("Fetching countries...");
    let countries = get_countries(&client).await?;
    info!("Found {} countries", countries.len());

    // Fetch regions concurrently
    let pb = mp.add(ProgressBar::new(countries.len() as u64));
    pb.set_style(style.clone());
    pb.set_message("Regions");

    let mut region_tasks = Vec::with_capacity(countries.len());
    for country in &countries {
        let client = client.clone();
        let pb = pb.clone();
        // Use Arc for shared ownership instead of unsafe if possible
        region_tasks.push(task::spawn(async move {
            let regions = get_regions(&client, country).await;
            pb.inc(1);
            regions
        }));
    }

    let mut regions = Vec::new();
    for task in region_tasks {
        regions.extend(task.await.unwrap());
    }
    pb.finish_with_message("Regions done");
    info!("Found {} regions", regions.len());

    // Fetch sub-regions concurrently
    let pb = mp.add(ProgressBar::new(regions.len() as u64));
    pb.set_style(style.clone());
    pb.set_message("Sub-regions");

    let mut sub_tasks = Vec::with_capacity(regions.len());
    for region in &regions {
        let client = client.clone();
        let pb = pb.clone();
        sub_tasks.push(task::spawn(async move {
            let subs = get_sub_regions(&client, region).await;
            pb.inc(1);
            subs
        }));
    }

    let mut sub_regions = Vec::new();
    for task in sub_tasks {
        sub_regions.extend(task.await.unwrap());
    }
    pb.finish_with_message("Sub-regions done");
    info!("Found {} sub-regions", sub_regions.len());

    let mut sub_region_df = sub_regions_to_df(&sub_regions)?;
    write_csv(&mut sub_region_df, "regions.csv")?;
    print_elapsed(&start, "Sub-regions scraped");

    // Fetch hotspots concurrently
    let hotspot_start = Instant::now();
    let pb = mp.add(ProgressBar::new(sub_regions.len() as u64));
    pb.set_style(style.clone());
    pb.set_message("Hotspots");

    let mut hotspot_tasks = Vec::with_capacity(sub_regions.len());
    for sub in &sub_regions {
        let client = client.clone();
        let pb = pb.clone();
        hotspot_tasks.push(task::spawn(async move {
            let spots = get_hotspots(&client, sub).await;
            pb.inc(1);
            spots
        }));
    }

    let mut hotspots = Vec::new();
    for task in hotspot_tasks {
        hotspots.extend(task.await.unwrap());
    }
    pb.finish_with_message("Hotspots done");
    info!("Found {} hotspots", hotspots.len());

    let mut hotspot_df = hotspots_to_df(&hotspots)?;
    write_csv(&mut hotspot_df, "hotspots.csv")?;
    print_elapsed(&hotspot_start, "Hotspots scraped");
    print_elapsed(&start, "Total time");

    Ok(())
}