#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub(crate) struct Country {
    country: String,
    country_code: String,
}

impl Country {
    pub(crate) fn new(country: &str, country_code: &str) -> Self {
        Self {
            country: country.to_string(),
            country_code: country_code.to_string(),
        }
    }

    pub(crate) fn country(&self) -> &str {
        &self.country
    }

    pub(crate) fn country_code(&self) -> &str {
        &self.country_code
    }
}

#[derive(Hash, Eq, PartialEq, Debug)]
pub(crate) struct Region<'a> {
    region: String,
    region_code: String,
    country: &'a Country,
}

impl<'a> Region<'a> {
    pub(crate) fn new(region: &str, region_code: &str, country: &'a Country) -> Self {
        Self {
            region: region.to_string(),
            region_code: region_code.to_string(),
            country,
        }
    }

    pub(crate) fn country(&self) -> &str {
        self.country.country()
    }

    pub(crate) fn country_code(&self) -> &str {
        self.country.country_code()
    }

    pub(crate) fn region(&self) -> &str {
        &self.region
    }

    pub(crate) fn region_code(&self) -> &str {
        &self.region_code
    }
}

#[derive(Hash, Eq, PartialEq, Debug)]
pub(crate) struct SubRegion<'a> {
    sub_region: String,
    sub_region_code: String,
    region: &'a Region<'a>,
}

impl<'a> SubRegion<'a> {
    pub(crate) fn new(sub_region: &str, sub_region_code: &str, region: &'a Region<'a>) -> Self {
        Self {
            sub_region: sub_region.to_string(),
            sub_region_code: sub_region_code.to_string(),
            region,
        }
    }

    pub(crate) fn country(&self) -> &str {
        self.region.country()
    }

    pub(crate) fn country_code(&self) -> &str {
        self.region.country_code()
    }

    pub(crate) fn region(&self) -> &str {
        self.region.region()
    }

    pub(crate) fn region_code(&self) -> &str {
        self.region.region_code()
    }

    pub(crate) fn sub_region(&self) -> &str {
        &self.sub_region
    }

    pub(crate) fn sub_region_code(&self) -> &str {
        &self.sub_region_code
    }
}

#[derive(Hash, Eq, PartialEq, Debug)]
pub(crate) struct Hotspot<'a> {
    hotspot: String,
    hotspot_code: String,
    sub_region: &'a SubRegion<'a>,
}

impl<'a> Hotspot<'a> {
    pub(crate) fn new(hotspot: &str, hotspot_code: &str, sub_region: &'a SubRegion<'a>) -> Self {
        Self {
            hotspot: hotspot.to_string(),
            hotspot_code: hotspot_code.to_string(),
            sub_region,
        }
    }

    pub(crate) fn country(&self) -> String {
        self.sub_region.country().to_string()
    }

    pub(crate) fn country_code(&self) -> String {
        self.sub_region.country_code().to_string()
    }

    pub(crate) fn hotspot(&self) -> &str {
        &self.hotspot
    }

    pub(crate) fn hotspot_code(&self) -> &str {
        &self.hotspot_code
    }

    pub(crate) fn region(&self) -> String {
        self.sub_region.region().to_string()
    }

    pub(crate) fn region_code(&self) -> String {
        self.sub_region.region_code().to_string()
    }

    pub(crate) fn sub_region(&self) -> String {
        self.sub_region.sub_region().to_string()
    }

    pub(crate) fn sub_region_code(&self) -> String {
        self.sub_region.sub_region_code().to_string()
    }
}
