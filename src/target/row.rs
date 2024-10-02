use crate::target::utils::remove_quote;
use polars::series::SeriesIter;

/// Struct containing location information of regions to be scraped.
///
/// Locations can have three or four levels:
/// - Country: The coarsest level.
/// - Region: The political level below country, usually equivalent to a state or county.
/// - Sub-region: One level below region, usually equivalent to a US county.
///
/// If a region does not have distinct sub-regions, `sub_region` has the same value as `region`.
/// For very small countries with no subdivisions, all three values are the same.
///
/// When scraping on the Hotspot level, the optional `hotspot` value is included.
#[derive(Debug, Clone)]
pub struct LocationRow {
    pub(crate) country: String,
    pub(crate) region: String,
    pub(crate) sub_region: String,
    pub(crate) hotspot: Option<String>,
}

impl LocationRow {
    /// Creates LocationRow from a vector of data extracted from a DataFrame row.
    pub(super) fn new(loc: &mut [SeriesIter]) -> Self {
        match loc.len() {
            3 => Self::new_location(
                remove_quote(&loc[0].next().unwrap().to_string()),
                remove_quote(&loc[1].next().unwrap().to_string()),
                remove_quote(&loc[2].next().unwrap().to_string()),
            ),
            4 => Self::new_hotspot(
                remove_quote(&loc[0].next().unwrap().to_string()),
                remove_quote(&loc[1].next().unwrap().to_string()),
                remove_quote(&loc[2].next().unwrap().to_string()),
                remove_quote(&loc[3].next().unwrap().to_string()),
            ),
            _ => panic!("Invalid number of location elements"),
        }
    }

    /// Creates LocationRow for hotspot location.
    pub(super) fn new_hotspot(
        country: String,
        region: String,
        sub_region: String,
        hotspot: String,
    ) -> Self {
        Self {
            country,
            region,
            sub_region,
            hotspot: Some(hotspot),
        }
    }

    /// Creates LocationRow for subregion location.
    pub(super) fn new_location(country: String, region: String, sub_region: String) -> Self {
        Self {
            country,
            region,
            sub_region,
            hotspot: None,
        }
    }

    /// Returns the LocationRow country.
    pub(super) fn country(&self) -> &str {
        &self.country
    }

    /// Returns the LocationRow region.
    pub(super) fn region(&self) -> &str {
        &self.region
    }

    /// Returns the LocationRow sub-region.
    pub(super) fn sub_region(&self) -> &str {
        &self.sub_region
    }

    /// Returns the LocationRow hotspot.
    pub(super) fn hotspot(&self) -> Option<&str> {
        self.hotspot.as_deref()
    }
}