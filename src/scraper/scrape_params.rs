/**
 Enumerates all possible geographic list types.

 If LocationLevel is Hotspot, then only Hotspot and Global are available. If ListType is Hotspot all
 species needed for that hotspot are extracted. If ListType is global, only those species not on the global
 life list are extracted.

 If LocationLevel is SubRegion, then SubRegion, Region, Country, and Global are available ListTypes. If ListType
 is SubRegin all species need for that sub-region are extracted. If ListType is Region, then only those species
 not already on that region's life list are extracted. In similar fashion, a ListType of Country restricts
 attention to species not already aquired for the country and a ListType of Global excludes all species already
 on the global life list.
 */
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
#[derive(Display, Debug, PartialEq)]
pub(crate) enum DateRange {
    #[strum(serialize = "life")]
    Life,
    #[strum(serialize = "year")]
    YTD,
    #[strum(serialize = "month")]
    MTD,
    #[strum(serialize = "day")]
    Day,
}
#[derive(Display, Debug, PartialEq)]
pub(crate) enum LocationLevel {
    #[strum(serialize = "sub_region_code")]
    SubRegion,
    #[strum(serialize = "hotspot_code")]
    Hotspot,
}
