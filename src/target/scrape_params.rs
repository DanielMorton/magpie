
/// Enumerates all possible geographic list types.
///
/// - For Hotspot LocationLevel: Only Hotspot and Global are available.
///   - Hotspot: All species needed for that hotspot are extracted.
///   - Global: Only species not on the global life list are extracted.
///
/// - For SubRegion LocationLevel: SubRegion, Region, Country, and Global are available.
///   - SubRegion: All species needed for that sub-region are extracted.
///   - Region: Only species not already on that region's life list are extracted.
///   - Country: Only species not already acquired for the country are extracted.
///   - Global: Excludes all species already on the global life list.
#[derive(Display, Debug, PartialEq)]
pub(crate) enum ListType {
    #[strum(serialize = "hotspot_code")]
    Hotspot,
    #[strum(serialize = "sub_region_code")]
    SubRegion,
    #[strum(serialize = "region_code")]
    Region,
    #[strum(serialize = "country_code")]
    Country,
    Global,
}

/// Enumerates all possible temporal list types.
///
/// - Life: All species for the location not on the life list are extracted (most common use case).
/// - Year: All species not on the year list are extracted.
/// - Month: Species not on the month list (for all previous years) are extracted.
/// - Date: Species not found on that calendar date (for all years) are extracted.
#[derive(Display, Debug, PartialEq)]
pub(crate) enum DateRange {
    #[strum(serialize = "life")]
    Life,
    #[strum(serialize = "year")]
    Year,
    #[strum(serialize = "month")]
    Month,
    #[strum(serialize = "day")]
    Date,
}

/// Enumerates the location levels for which data can be extracted.
///
/// The program is provided with a list of locations, which can either be hotspots or sub-regions.
/// LocationLevel tracks which type of region target species should be extracted from.
#[derive(Display, Debug, PartialEq)]
pub(crate) enum LocationLevel {
    #[strum(serialize = "sub_region_code")]
    SubRegion,
    #[strum(serialize = "hotspot_code")]
    Hotspot,
}