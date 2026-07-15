extern crate strum;
#[macro_use]
extern crate strum_macros;
mod loc;
mod location;
mod login;
mod run_location;
mod run_scraper;
mod selector_macro;
mod target;

use std::error::Error;

use crate::target::SpeciesArgs;
use clap::{Parser, Subcommand}; // Added the necessary imports

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "location")]
    Location,
    #[command(name = "species")]
    Species(SpeciesArgs),
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Species(args) => run_scraper::run(args),
        Commands::Location => run_location::run(),
    }
}
