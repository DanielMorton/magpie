use crate::app::AppType;
use crate::target::scrape_params::LocationLevel::{Hotspot, SubRegion};
use crate::target::scrape_params::{DateRange, ListType, LocationLevel};
use clap::{arg, value_parser, Arg, ArgGroup, ArgMatches, Command};

static DEFAULT_LOCATION: &str = "regions.csv";

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

pub(crate) trait MagpieParse {
    fn get_app(&self) -> AppType;
    fn get_date_range(&self) -> DateRange;
    fn get_list_type(&self) -> ListType;
    fn get_loc_data(&self) -> (&str, LocationLevel);
    fn get_output_file(&self) -> &str;
    fn get_time_range(&self) -> Vec<(u8, u8)>;
}

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

    fn get_date_range(&self) -> DateRange {
        match (
            self.get_flag("life"),
            self.get_flag("ytd"),
            self.get_flag("current_month"),
            self.get_flag("date"),
        ) {
            (true, _, _, _) => DateRange::Life,
            (_, true, _, _) => DateRange::Year,
            (_, _, true, _) => DateRange::Month,
            (_, _, _, true) => DateRange::Date,
            _ => panic!("Invalid Date Range."),
        }
    }

    fn get_list_type(&self) -> ListType {
        if self.get_one::<String>("hotspot").is_some() {
            return if self.get_flag("local") {
                ListType::Hotspot
            } else if self.get_flag("global") {
                ListType::Global
            } else {
                panic!("Invalid List Type for hotspot list.")
            };
        }

        match self.get_one::<String>("subregion") {
            Some(_) => match (
                self.get_flag("local"),
                self.get_flag("region"),
                self.get_flag("country"),
                self.get_flag("global"),
            ) {
                (true, _, _, _) => ListType::SubRegion,
                (_, true, _, _) => ListType::Region,
                (_, _, true, _) => ListType::Country,
                (_, _, _, true) => ListType::Global,
                _ => panic!("Invalid List Type for geo list."),
            },
            None => panic!("Invalid List Type."),
        }
    }

    fn get_loc_data(&self) -> (&str, LocationLevel) {
        self.get_one::<String>("hotspot")
            .map(|f| (f.as_str(), Hotspot))
            .or_else(|| {
                self.get_one::<String>("subregion")
                    .map(|f| (f.as_str(), SubRegion))
            })
            .unwrap_or((DEFAULT_LOCATION, SubRegion))
    }

    fn get_output_file(&self) -> &str {
        self.get_one::<String>("output")
            .expect("Missing output file.")
    }

    fn get_time_range(&self) -> Vec<(u8, u8)> {
        if self.get_flag("year") {
            vec![(1, 12)]
        } else if self.get_flag("all") {
            (1..=12).map(|m| (m, m)).collect()
        } else {
            self.get_one::<u8>("month")
                .map(|&m| vec![(m, m)])
                .or_else(|| {
                    self.get_one::<String>("range").map(|range| {
                        let r: Vec<&str> = range.split('-').collect();
                        let start_month = r[0].parse::<u8>().expect("Invalid start month");
                        let end_month = r[1].parse::<u8>().expect("Invalid end month");
                        vec![(start_month, end_month)]
                    })
                })
                .expect("Invalid time range.")
        }
    }
}
