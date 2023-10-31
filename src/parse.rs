use crate::scrape_params::ListLevel::{Hotspot, SubRegion};
use crate::scrape_params::{DateRange, ListLevel, ListType};
use clap::{arg, ArgGroup, ArgMatches, Command};

static DEFAULT_LOCATION: &str = "regions.csv";

pub(crate) fn parse() -> ArgMatches {
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
        .arg(arg!(--month <NUM>))
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
        .arg(arg!(--num_cores[NUM_CORES]))
        .get_matches()
}

pub(crate) trait MagpieParse {
    fn get_date_range(&self) -> DateRange;

    fn get_list_type(&self) -> ListType;

    fn get_loc_data(&self) -> (&str, ListLevel);

    fn get_num_cores(&self) -> u8;

    fn get_output_file(&self) -> &str;

    fn get_time_range(&self) -> Vec<(u8, u8)>;
}

impl MagpieParse for ArgMatches {
    fn get_date_range(&self) -> DateRange {
        if self.get_flag("life") {
            DateRange::Life
        } else if self.get_flag("ytd") {
            DateRange::YTD
        } else if self.get_flag("mtd") {
            DateRange::MTD
        } else if self.get_flag("day ") {
            DateRange::Day
        } else {
            panic!("Invalid Date Range.")
        }
    }

    fn get_list_type(&self) -> ListType {
        if self.get_flag("local") {
            ListType::Local
        } else if self.get_flag("region") {
            ListType::Region
        } else if self.get_flag("country") {
            ListType::Country
        } else if self.get_flag("global") {
            ListType::Global
        } else {
            panic!("Invalid List Type.")
        }
    }

    fn get_loc_data(&self) -> (&str, ListLevel) {
        match self.get_one::<String>("hotspot") {
            Some(f) => (f, Hotspot),
            None => match self.get_one::<String>("geo") {
                Some(f) => (f, SubRegion),
                None => (DEFAULT_LOCATION, SubRegion),
            },
        }
    }

    fn get_num_cores(&self) -> u8 {
        match self.get_one::<u8>("num_cores") {
            Some(&c) => c,
            None => {
                let cores = num_cpus::get() as u8;
                if cores > 1 {
                    cores / 2
                } else {
                    cores
                }
            }
        }
    }

    fn get_output_file(&self) -> &str {
        match self.get_one::<String>("output") {
            Some(output) => output,
            None => panic!("Missing output file."),
        }
    }

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
