use crate::error::{AppError, Result};
use crate::target::scrape_params::LocationLevel::{Hotspot, SubRegion};
use crate::target::scrape_params::{DateRange, ListType, LocationLevel};
use clap::{ArgGroup, Args, Parser};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct SpeciesArgs {
    #[command(flatten)]
    pub list_options: ListOptions,

    #[command(flatten)]
    pub date_options: DateOptions,

    #[command(flatten)]
    pub time_options: TimeOptions,

    #[command(flatten)]
    pub location_options: LocationOptions,

    /// Specify output format
    #[arg(long, value_name = "OUTPUT")]
    pub output: Option<String>,
}

impl SpeciesArgs {
    pub fn get_list_type(&self, location: &LocationOptions) -> Result<ListType> {
        if location.hotspot.is_some() {
            if self.list_options.local {
                Ok(ListType::Hotspot)
            } else if self.list_options.global {
                Ok(ListType::Global)
            } else {
                Err(AppError::InvalidListType)
            }
        } else {
            match location.subregion {
                Some(_) => match (
                    self.list_options.local,
                    self.list_options.region,
                    self.list_options.country,
                    self.list_options.global,
                ) {
                    (true, _, _, _) => Ok(ListType::SubRegion),
                    (_, true, _, _) => Ok(ListType::Region),
                    (_, _, true, _) => Ok(ListType::Country),
                    (_, _, _, true) => Ok(ListType::Global),
                    _ => Err(AppError::InvalidListType),
                },
                None => Err(AppError::MissingLocation),
            }
        }
    }

    pub fn get_date_range(&self) -> Result<DateRange> {
        match (
            self.date_options.life,
            self.date_options.ytd,
            self.date_options.current_month,
            self.date_options.date,
        ) {
            (true, _, _, _) => Ok(DateRange::Life),
            (_, true, _, _) => Ok(DateRange::Year),
            (_, _, true, _) => Ok(DateRange::Month),
            (_, _, _, true) => Ok(DateRange::Date),
            _ => Err(AppError::Parse("Date range not specified".into())),
        }
    }

    pub fn get_time_range(&self) -> Result<Vec<(u8, u8)>> {
        self.time_options.get_time_range()
    }

    pub fn get_loc_data(&self) -> Result<(&str, LocationLevel)> {
        self.location_options.get_loc_data()
    }

    pub fn get_output_file(&self) -> Result<&str> {
        self.output.as_deref().ok_or(AppError::MissingOutputFile)
    }
}

#[derive(Args)]
#[command(group(
    ArgGroup::new("list_type")
        .required(true)
        .args(["local", "region", "country", "global"]),
))]
pub struct ListOptions {
    #[arg(long)]
    pub local: bool,
    #[arg(long)]
    pub region: bool,
    #[arg(long)]
    pub country: bool,
    #[arg(long)]
    pub global: bool,
}

#[derive(Args)]
#[command(group(
    ArgGroup::new("date_range")
        .required(true)
        .args(["life", "ytd", "current_month", "date"]),
))]
pub struct DateOptions {
    #[arg(long)]
    pub life: bool,
    #[arg(long)]
    pub ytd: bool,
    #[arg(long)]
    pub current_month: bool,
    #[arg(long)]
    pub date: bool,
}

#[derive(Args)]
#[command(group(
    ArgGroup::new("time_range")
        .required(true)
        .args(["year", "month", "all", "range"]),
))]
pub struct TimeOptions {
    #[arg(long)]
    pub year: bool,
    #[arg(long)]
    pub all: bool,
    #[arg(long, value_parser = clap::value_parser!(u8).range(1..=12))]
    pub month: Option<u8>,
    #[arg(long, value_name = "RANGE")]
    pub range: Option<String>,
}

impl TimeOptions {
    fn get_time_range(&self) -> Result<Vec<(u8, u8)>> {
        if self.year {
            return Ok(vec![(1, 12)]);
        }
        if self.all {
            return Ok((1..=12).map(|m| (m, m)).collect());
        }
        if let Some(month) = self.month {
            return Ok(vec![(month, month)]);
        }
        if let Some(range) = &self.range {
            let parts: Vec<&str> = range.split('-').collect();
            if parts.len() != 2 {
                return Err(AppError::InvalidTimeRange(range.clone()));
            }
            let start = parts[0].parse::<u8>()?;
            let end = parts[1].parse::<u8>()?;
            if !(1..=12).contains(&start) || !(1..=12).contains(&end) {
                return Err(AppError::InvalidTimeRange(range.clone()));
            }
            return Ok(vec![(start, end)]);
        }
        Err(AppError::Parse("Time range not specified".into()))
    }
}

#[derive(Args)]
#[command(group(
    ArgGroup::new("location_type")
        .required(true)
        .args(["hotspot", "subregion"]),
))]
pub struct LocationOptions {
    #[arg(long, value_name = "HOTSPOT")]
    pub hotspot: Option<String>,
    #[arg(long, value_name = "SUBREGION")]
    pub subregion: Option<String>,
}

impl LocationOptions {
    fn get_loc_data(&self) -> Result<(&str, LocationLevel)> {
        self.hotspot
            .as_ref()
            .map(|f| (f.as_str(), Hotspot))
            .or_else(|| self.subregion.as_ref().map(|f| (f.as_str(), SubRegion)))
            .ok_or(AppError::MissingLocation)
    }
}
