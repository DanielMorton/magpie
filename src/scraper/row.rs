use polars::series::SeriesIter;
use crate::scraper::utils::remove_quote;


#[derive(Debug, Clone)]
/**
Struct containing location information of regions to scraped. Locations can have three or four levels.
Country is the coarsest level and is self-explanitory. Region is the political level below country and usually
equivalent to a state or, for small countries, a county. Sub-region is one level below region, usually equivalent
to a US county.

If a region does not not have distinct sub-regions, sub_region has the same value as region. In the case of very
small countries with no subdivisions, all three values are the same.

When scraping on the Hotspot level, the optional hotpsot value is included. Othervise hotspot is None.
 */

pub struct LocationRow {

    /// Country of location to be scraped.
    pub(crate) country: String,

    /// Region of location to be scraped. Same as country if country has no subdivisions.
    pub(crate) region: String,

    /// Sub-region of locaton to be scraped. Same as region if region has no subdivisions.
    pub(crate) sub_region: String,

    /// Hotspot to be scraped. Only used if scraping is on the hotpsot level.
    pub(crate) hotspot: Option<String>,
}

/// Implementation of LocationRow.
impl LocationRow {

    /// Creates LocationRow from a vector of data extracted from a DataFrame row.
    pub(crate) fn new(loc: &mut Vec<SeriesIter>) -> Self {

        if loc.len() == 3 {
            LocationRow::new_location(remove_quote(&loc[0].next().unwrap().to_string()),
                                      remove_quote(&loc[1].next().unwrap().to_string()),
                                      remove_quote(&loc[2].next().unwrap().to_string()))
        } else {
            LocationRow::new_hotspot(
                remove_quote(&loc[0].next().unwrap().to_string()),
                remove_quote(&loc[1].next().unwrap().to_string()),
                remove_quote(&loc[2].next().unwrap().to_string()),
                remove_quote(&loc[3].next().unwrap().to_string()),
            )
        }
    }

    /// Creates LocationRow for hotspot location.
    pub(crate) fn new_hotspot(
        country: String,
        region: String,
        sub_region: String,
        hotspot: String,
    ) -> Self {
        LocationRow {
            country,
            region,
            sub_region,
            hotspot: Some(hotspot),
        }
    }

    /// Creates LocationRow for subregion location.
    pub(crate) fn new_location(country: String, region: String, sub_region: String) -> Self {
        LocationRow {
            country,
            region,
            sub_region,
            hotspot: None,
        }
    }

    /// Returns the LocationRow country.
    pub(crate) fn country(&self) -> &str {
        self.country.as_str()
    }

    /// Returns the LocationRow region.
    pub(crate) fn region(&self) -> &str { self.region.as_str() }

    /// Returns the LocationRow sub-region.
    pub(crate) fn sub_region(&self) -> &str {
        self.sub_region.as_str()
    }

    /// Returns the LocationRow hotspot.
    pub(crate) fn hotspot(&self) -> Option<&str> { self.hotspot.as_ref().map(|s| s.as_str()) }
}


pub struct SpeciesRow<'a> {
    pub(crate) common_name: &'a str,
    pub(crate) scientific_name: &'a str,
    pub(crate) percent: f32,
}

impl<'a> SpeciesRow<'a> {
    pub(crate) fn new(common_name: &'a str, scientific_name: &'a str, percent: f32) -> Self {
        SpeciesRow {
            common_name,
            scientific_name,
            percent,
        }
    }
}
