use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ListType {
    Hotspot,
    SubRegion,
    Region,
    Country,
    Global,
}

impl fmt::Display for ListType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ListType::Hotspot => write!(f, "hotspot"),
            ListType::SubRegion => write!(f, "sub_region"),
            ListType::Region => write!(f, "region"),
            ListType::Country => write!(f, "country"),
            ListType::Global => write!(f, "global"),
        }
    }
}

impl FromStr for ListType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "hotspot" => Ok(ListType::Hotspot),
            "sub_region" => Ok(ListType::SubRegion),
            "region" => Ok(ListType::Region),
            "country" => Ok(ListType::Country),
            "global" => Ok(ListType::Global),
            _ => Err(format!("unknown ListType: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DateRange {
    Life,
    Year,
    Month,
    Date,
}

impl fmt::Display for DateRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DateRange::Life => write!(f, "life"),
            DateRange::Year => write!(f, "year"),
            DateRange::Month => write!(f, "month"),
            DateRange::Date => write!(f, "date"),
        }
    }
}

impl FromStr for DateRange {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "life" => Ok(DateRange::Life),
            "year" => Ok(DateRange::Year),
            "month" => Ok(DateRange::Month),
            "date" => Ok(DateRange::Date),
            _ => Err(format!("unknown DateRange: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LocationLevel {
    SubRegion,
    Hotspot,
}

impl fmt::Display for LocationLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LocationLevel::SubRegion => write!(f, "sub_region"),
            LocationLevel::Hotspot => write!(f, "hotspot"),
        }
    }
}

impl FromStr for LocationLevel {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "sub_region" => Ok(LocationLevel::SubRegion),
            "hotspot" => Ok(LocationLevel::Hotspot),
            _ => Err(format!("unknown LocationLevel: {}", s)),
        }
    }
}
