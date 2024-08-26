use crate::app::AppType;
use crate::target::scrape_params::LocationLevel::{Hotspot, SubRegion};
use crate::target::scrape_params::{DateRange, ListType, LocationLevel};
use clap::{arg, value_parser, Arg, ArgGroup, ArgMatches, Command};

static DEFAULT_LOCATION: &str = "regions.csv";

/**
 Returns all the command line inputs. These are the list type, date range, time range (the number
 of months covered) and the type of location (sub-region or hotspot) for which data is extracted.
 The location tag accepts as input the csv file containing the locations for which data is to be
 scraped. An output csv file must also be specified.
*/
pub(crate) fn parse() -> ArgMatches {
    Command::new("magpie")
        .arg(arg!(--species))
        .arg(arg!(--location))
        .group(
            ArgGroup::new("app")
                .args(["species", "location"])
                .required(true),
        )
        .arg(arg!(--local))
        .arg(arg!(--region))
        .arg(arg!(--country))
        .arg(arg!(--global))
        .group(ArgGroup::new("list_type").args(["local", "region", "country", "global"]))
        .arg(arg!(--life))
        .arg(arg!(--ytd))
        .arg(arg!(--current_month))
        .arg(arg!(--date))
        .group(ArgGroup::new("date_range").args(["life", "ytd", "current_month", "date"]))
        .arg(arg!(--year))
        .arg(arg!(--all))
        .arg(
            Arg::new("month")
                .long("month")
                .required(false)
                .value_parser(value_parser!(u8)),
        )
        .arg(arg!(--range <RANGE>))
        .group(ArgGroup::new("time_range").args(["year", "month", "all", "range"]))
        .arg(arg!(--hotspot <HOTSPOT>))
        .arg(arg!(--subregion <SUBREGION>))
        .group(ArgGroup::new("list-type").args(["hotspot", "subregion"]))
        .arg(arg!(--output <OUTPUT>))
        .get_matches()
}

/**
 Trait for parsing the magpie specific command line arguments.
*/
pub(crate) trait MagpieParse {
    fn get_app(&self) -> AppType;

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
    fn get_app(&self) -> AppType {
        if self.get_flag("species") {
            AppType::Species
        } else if self.get_flag("location") {
            AppType::Location
        } else {
            panic!("Invalid selection for application.")
        }
    }

    /// Extracts the DateRange.
    fn get_date_range(&self) -> DateRange {
        if self.get_flag("life") {
            DateRange::Life
        } else if self.get_flag("ytd") {
            DateRange::Year
        } else if self.get_flag("current_month") {
            DateRange::Month
        } else if self.get_flag("date") {
            DateRange::Date
        } else {
            panic!("Invalid Date Range.")
        }
    }

    /// Extracts the ListType
    fn get_list_type(&self) -> ListType {
        if  self.get_one::<String>("hotspot").is_some() {
            if self.get_flag("local") {
                return ListType::Hotspot;
            } else if self.get_flag("global") {
                return ListType::Global;
            } else {
                panic!("Invalid List Type for hotspot list.")
            }
        };
        match self.get_one::<String>("subregion") {
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
            None => match self.get_one::<String>("subregion") {
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
