use strum_macros::{Display, EnumString};

#[derive(Display, Debug, Clone, Copy, PartialEq, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum ListType {
    Hotspot,
    SubRegion,
    Region,
    Country,
    Global,
}

#[derive(Display, Debug, Clone, Copy, PartialEq, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum DateRange {
    Life,
    Year,
    Month,
    Date,
}

#[derive(Display, Debug, Clone, Copy, PartialEq, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum LocationLevel {
    SubRegion,
    Hotspot,
}