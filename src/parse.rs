use crate::scraper::scrape_params::LocationLevel::{Hotspot, SubRegion};
use crate::scraper::scrape_params::{DateRange, ListType, LocationLevel};
use clap::{arg, value_parser, Arg, ArgGroup, ArgMatches, Command};

static DEFAULT_LOCATION: &str = "regions.csv";

/**
 Returns all the command line inputs. These are the list type, date range, time range (the number
 of months covered) and the type of location (sub-region or hotspot) for which data is extracted.
 The location tag accepts as input the csv file containing the locations for which data is to be
 scraped. An output csv file must also be specified.
*/
pub(super) fn parse() -> ArgMatches {
    Command::new("magpie")
        .arg(arg!(--local))
        .arg(arg!(--region))
        .arg(arg!(--country))
        .arg(arg!(--global))
        .group(
            ArgGroup::new("list_type")
                .args(["local", "region", "country", "global"])
                .required(true),
        )
        .arg(arg!(--life))
        .arg(arg!(--ytd))
        .arg(arg!(--mtd))
        .arg(arg!(--day))
        .group(
            ArgGroup::new("date_range")
                .args(["life", "ytd", "mtd", "day"])
                .required(true),
        )
        .arg(arg!(--year))
        .arg(arg!(--all))
        .arg(
            Arg::new("month")
                .long("month")
                .required(false)
                .value_parser(value_parser!(u8)),
        )
        .arg(arg!(--range <RANGE>))
        .group(
            ArgGroup::new("time_range")
                .args(["year", "month", "all", "range"])
                .required(true),
        )
        .arg(arg!(--hotspot[HOTSPOT]))
        .arg(arg!(--geo[GEO]))
        .group(ArgGroup::new("csv_range").args(["hotspot", "region"]))
        .arg(arg!(--output <OUTPUT>))
        .get_matches()
}

/**
 Trait for parsing the magpie specific command line arguments.
*/
pub(super) trait MagpieParse {
    /// Extracts the DateRange.
    fn get_date_range(&self) -> DateRange;

    /// Extracts the ListType
    fn get_list_type(&self) -> ListType;

    /// Extracts the name of the file containing the locations, returning both the file name and the
    /// type of the locations (sub-region or hotspot.)
    fn get_loc_data(&self) -> (&str, LocationLevel);

    /// Extracts the name of the output file.
    fn get_output_file(&self) -> &str;

    /// Extracts the time range, either a full year, each month individually, or a range of months.
    fn get_time_range(&self) -> Vec<(u8, u8)>;
}

/**
 Implementation of MagpieParse for ArgMatches.
*/
impl MagpieParse for ArgMatches {
    /// Extracts the DateRange.
    fn get_date_range(&self) -> DateRange {
        if self.get_flag("life") {
            DateRange::Life
        } else if self.get_flag("ytd") {
            DateRange::YTD
        } else if self.get_flag("mtd") {
            DateRange::Month
        } else if self.get_flag("day ") {
            DateRange::Day
        } else {
            panic!("Invalid Date Range.")
        }
    }

    /// Extracts the ListType
    fn get_list_type(&self) -> ListType {
        match self.get_one::<String>("hotspot") {
            Some(_) => {
                if self.get_flag("local") {
                    return ListType::Hotspot;
                } else if self.get_flag("global") {
                    return ListType::Global;
                } else {
                    panic!("Invalid List Type for hotspot list.")
                }
            }
            None => (),
        };
        match self.get_one::<String>("geo") {
            Some(_) => {
                if self.get_flag("local") {
                    ListType::SubRegion
                } else if self.get_flag("region") {
                    ListType::Region
                } else if self.get_flag("country") {
                    ListType::Country
                } else if self.get_flag("global") {
                    ListType::Global
                } else {
                    panic!("Invalid List Type for geo list.")
                }
            }
            None => panic!("Invalid List Type."),
        }
    }

    /// Extracts the name of the file containing the locations, returning both the file name and the
    /// type of the locations (sub-region or hotspot.)
    fn get_loc_data(&self) -> (&str, LocationLevel) {
        match self.get_one::<String>("hotspot") {
            Some(f) => (f, Hotspot),
            None => match self.get_one::<String>("geo") {
                Some(f) => (f, SubRegion),
                None => (DEFAULT_LOCATION, SubRegion),
            },
        }
    }

    /// Extracts the name of the output file.
    fn get_output_file(&self) -> &str {
        match self.get_one::<String>("output") {
            Some(output) => output,
            None => panic!("Missing output file."),
        }
    }

    /// Extracts the time range, either a full year, each month individually, or a range of months.
    fn get_time_range(&self) -> Vec<(u8, u8)> {
        if self.get_flag("year") {
            vec![(1, 12)]
        } else if self.get_flag("all") {
            (1..=12).map(|m| (m, m)).collect()
        } else {
            match self.get_one::<u8>("month") {
                Some(&m) => vec![(m, m)],
                None => match self.get_one::<String>("range") {
                    Some(range) => {
                        let r: Vec<&str> = range.split('-').collect();
                        let start_month = match r[0].parse::<u8>() {
                            Ok(m) => m,
                            Err(e) => panic!("{}", e),
                        };
                        let end_month = match r[0].parse::<u8>() {
                            Ok(m) => m,
                            Err(e) => panic!("{}", e),
                        };
                        vec![(start_month, end_month)]
                    }
                    None => panic!("Invalid time range."),
                },
            }
        }
    }
}
