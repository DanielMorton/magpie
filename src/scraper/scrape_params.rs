#[derive(Display, Debug, PartialEq)]
pub(crate) enum ListType {
    #[strum(serialize = "hotspot")]
    Hotspot,
    #[strum(serialize = "sub_region")]
    Local,
    #[strum(serialize = "region")]
    Region,
    #[strum(serialize = "country")]
    Country,
    Global,
}

impl ListType {
    pub(super) fn to_code(&self) -> String {
        self.to_string() + "_code"
    }
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
pub(crate) enum ListLevel {
    #[strum(serialize = "sub_region")]
    SubRegion,
    #[strum(serialize = "hotspot")]
    Hotspot,
}

impl ListLevel {
    pub(super) fn to_code(&self) -> String {
        self.to_string() + "_code"
    }
}
