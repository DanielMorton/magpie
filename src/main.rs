extern crate strum;
#[macro_use]
extern crate strum_macros;

mod app;
mod loc;
mod location;
mod login;
mod parse;
mod run_location;
mod run_scraper;
mod target;

use std::error::Error;

use crate::app::AppType;
use crate::parse::MagpieParse;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = parse::parse();
    match matches.get_app() {
        AppType::Species => run_scraper::run(&matches),
        AppType::Location => run_location::run()
    }
}
