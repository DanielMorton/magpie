mod app;
mod loc;
mod location;
mod login;
mod parse;
mod run_location;
mod run_scraper;
mod scraper;

extern crate strum;
#[macro_use]
extern crate strum_macros;

use crate::app::AppType;
use parse::MagpieParse;

fn main() {
    let matches = parse::parse();
    let app = matches.get_app();
    if app == AppType::Species {
        run_scraper::run(&matches)
    } else if app == AppType::Location {
        run_location::run()
    }
}
