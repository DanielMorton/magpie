use crate::target::error::ParseError;
use crate::target::error::ParseError::{InvalidBeginMonth, InvalidDateRange, InvalidEndMonth, InvalidLocationType, InvalidMonth, InvalidTimeOption, InvalidTimeRange, MissingOutputFile};
use crate::target::scrape_params::LocationLevel::{Hotspot, SubRegion};
use crate::target::scrape_params::{DateRange, ListType, LocationLevel};
use clap::{ArgGroup, Args, Parser};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct SpeciesArgs {
    #[command(flatten)]
    list_options: ListOptions,

    #[command(flatten)]
    date_options: DateOptions,

    #[command(flatten)]
    time_options: TimeOptions,

    #[command(flatten)]
    location_options: LocationOptions,

    /// Specify output format
    #[arg(long, value_name = "OUTPUT")]
    output: Option<String>,
}

impl SpeciesArgs {
    pub fn get_list_type(&self) -> Result<ListType, ParseError> {
        self.list_options.get_list_type(&self.location_options)
    }

    pub fn get_date_range(&self) -> Result<DateRange, ParseError> {
        self.date_options.get_date_range()
    }

    pub fn get_time_range(&self) -> Result<Vec<(u8, u8)>, ParseError> {
        self.time_options.get_time_range()
    }

    pub fn get_loc_data(&self) -> Result<(&str, LocationLevel), ParseError> {
        self.location_options.get_loc_data()
    }

    pub fn get_output_file(&self) -> Result<&str, ParseError> {
        self.output.as_deref().ok_or(MissingOutputFile)
    }
}

#[derive(Args)]
#[command(group(
ArgGroup::new("list_type")
.required(true)
.args(["local", "region", "country", "global"]),
))]
struct ListOptions {
    /// Show local list
    #[arg(long)]
    local: bool,

    /// Show region list
    #[arg(long)]
    region: bool,

    /// Show country list
    #[arg(long)]
    country: bool,

    /// Show global list
    #[arg(long)]
    global: bool,
}

impl ListOptions {
    fn get_list_type(&self, location: &LocationOptions) -> Result<ListType, ParseError> {
        if location.hotspot.is_some() {
            if self.local {
                Ok(ListType::Hotspot)
            } else if self.global {
                Ok(ListType::Global)
            } else {
                Err(ParseError::InvalidListType(Hotspot))
            }
        } else {
            match location.subregion {
                Some(_) => match (self.local, self.region, self.country, self.global) {
                    (true, _, _, _) => Ok(ListType::SubRegion),
                    (_, true, _, _) => Ok(ListType::Region),
                    (_, _, true, _) => Ok(ListType::Country),
                    (_, _, _, true) => Ok(ListType::Global),
                    _ => Err(ParseError::InvalidListType(SubRegion)),
                },
                None => Err(ParseError::InvalidLocationType),
            }
        }
    }
}

#[derive(Args)]
#[command(group(
ArgGroup::new("date_range")
.required(true)
.args(["life", "ytd", "current_month", "date"]),
))]
struct DateOptions {
    /// Show lifetime data
    #[arg(long, required = false)]
    life: bool,

    /// Show year-to-date data
    #[arg(long, required = false)]
    ytd: bool,

    /// Show current month data
    #[arg(long, required = false)]
    current_month: bool,

    /// Show data for specific date
    #[arg(long, required = false)]
    date: bool,
}

impl DateOptions {
    fn get_date_range(&self) -> Result<DateRange, ParseError> {
        match (self.life, self.ytd, self.current_month, self.date) {
            (true, _, _, _) => Ok(DateRange::Life),
            (_, true, _, _) => Ok(DateRange::Year),
            (_, _, true, _) => Ok(DateRange::Month),
            (_, _, _, true) => Ok(DateRange::Date),
            _ => Err(InvalidDateRange),
        }
    }
}

#[derive(Args)]
#[command(group(
ArgGroup::new("time_range")
.required(true)
.args(["year", "month", "all", "range"]),
))]
struct TimeOptions {
    /// Show yearly data
    #[arg(long, required = false)]
    year: bool,

    /// Show all data
    #[arg(long, required = false)]
    all: bool,

    /// Specify month (1-12)
    #[arg(long, value_parser = clap::value_parser!(u8).range(1..=12), required=false)]
    month: Option<u8>,

    /// Specify custom range
    #[arg(long, value_name = "RANGE", required = false)]
    range: Option<String>,
}
impl TimeOptions {
    fn get_time_range(&self) -> Result<Vec<(u8, u8)>, ParseError> {
        if self.year {
            return Ok(vec![(1, 12)]);
        }

        if self.all {
            return Ok((1..=12).map(|m| (m, m)).collect());
        }

        if let Some(month) = self.month {
            if !(1..=12).contains(&month) {
                return Err(InvalidMonth(month));
            }
            return Ok(vec![(month, month)]);
        }

        if let Some(range) = &self.range {
            let parts: Vec<&str> = range.split('-').collect();
            if parts.len() != 2 {
                return Err(InvalidTimeRange(range.to_owned()));
            }

            let start_month = parts[0].parse::<u8>()?;

            if !(1..=12).contains(&start_month) {
                return Err(InvalidBeginMonth(start_month.to_string()))
            }

            let end_month = parts[1].parse::<u8>()?;

            if !(1..=12).contains(&end_month) {
                return Err(InvalidEndMonth(end_month.to_string()))
            }

            return Ok(vec![(start_month, end_month)]);
        }
        Err(InvalidTimeOption)
    }
}

#[derive(Args)]
#[command(group(
ArgGroup::new("location_type")
.required(true)
.args(["hotspot", "subregion"]),
))]
struct LocationOptions {
    /// Specify hotspot
    #[arg(long, value_name = "HOTSPOT", required = false)]
    hotspot: Option<String>,

    /// Specify subregion
    #[arg(long, value_name = "SUBREGION", required = false)]
    subregion: Option<String>,
}

impl LocationOptions {
    fn get_loc_data(&self) -> Result<(&str, LocationLevel), ParseError> {
        self
            .hotspot
            .as_ref()
            .map(|f| (f.as_str(), Hotspot))
            .or_else(|| self.subregion.as_ref().map(|f| (f.as_str(), SubRegion)))
            .ok_or(InvalidLocationType)
    }
}
