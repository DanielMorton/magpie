mod error;
mod location;
mod login;
mod run_location;
mod run_scraper;
mod selectors;
mod target;
mod utils;

use clap::{Parser, Subcommand};
use std::process;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Location,
    Species(target::SpeciesArgs),
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();
    let result = match cli.command {
        Commands::Species(args) => run_scraper::run(args).await,
        Commands::Location => run_location::run().await,
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}