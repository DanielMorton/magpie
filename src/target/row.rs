use crate::target::error::LocationError;
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
    pub(super) fn new(loc: &mut [SeriesIter]) -> Result<Self, LocationError> {
        fn next_value(iter: &mut SeriesIter) -> String {
            remove_quote(
                &iter
                    .next()
                    .expect("SeriesIter should yield one value per DataFrame row")
                    .to_string(),
            )
        }

        match loc.len() {
            3 => Ok(Self::new_location(
                next_value(&mut loc[0]),
                next_value(&mut loc[1]),
                next_value(&mut loc[2]),
            )),
            4 => Ok(Self::new_hotspot(
                next_value(&mut loc[0]),
                next_value(&mut loc[1]),
                next_value(&mut loc[2]),
                next_value(&mut loc[3]),
            )),
            n => Err(LocationError::InvalidElementCount(n)),
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
