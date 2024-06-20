#[derive(Hash, Eq, PartialEq, Debug)]
pub(crate) struct Country {
    pub(crate) country: String,
    pub(crate) country_code: String,
}

impl Country {
    pub(crate) fn new(country: String, country_code: String) -> Self {
        Country {
            country,
            country_code,
        }
    }
}

#[derive(Hash, Eq, PartialEq, Debug)]
pub(crate) struct Region<'a> {
    pub(crate) region: String,
    pub(crate) region_code: String,
    pub(crate) country: &'a Country,
}

impl<'a> Region<'a> {
    pub(crate) fn new(region: String, region_code: String, country: &'a Country) -> Self {
        Region {
            region,
            region_code,
            country,
        }
    }

    pub(crate) fn country(&self) -> String {
        self.country.country.to_owned()
    }
    pub(crate) fn country_code(&self) -> String {
        self.country.country_code.to_owned()
    }
}

#[derive(Hash, Eq, PartialEq, Debug)]
pub(crate) struct SubRegion<'a> {
    pub(crate) sub_region: String,
    pub(crate) sub_region_code: String,
    pub(crate) region: &'a Region<'a>,
}

impl<'a> SubRegion<'a> {
    pub(crate) fn new(sub_region: String, sub_region_code: String, region: &'a Region<'a>) -> Self {
        SubRegion {
            sub_region,
            sub_region_code,
            region,
        }
    }

    pub(crate) fn country(&self) -> String {
        self.region.country()
    }
    pub(crate) fn country_code(&self) -> String {
        self.region.country_code()
    }
    pub(crate) fn region(&self) -> String {
        self.region.region.to_owned()
    }
    pub(crate) fn region_code(&self) -> String {
        self.region.region_code.to_owned()
    }
}
#[derive(Hash, Eq, PartialEq, Debug)]
pub(crate) struct Hotspot<'a> {
    pub(crate) hotspot: String,
    pub(crate) hotspot_code: String,
    pub(crate) sub_region: &'a SubRegion<'a>,
}

impl<'a> Hotspot<'a> {
    pub(crate) fn new(hotspot: String, hotspot_code: String, sub_region: &'a SubRegion<'a>) -> Self {
        Hotspot {
            hotspot,
            hotspot_code,
            sub_region,
        }
    }

    pub(crate) fn country(&self) -> String {
        self.sub_region.country()
    }
    pub(crate) fn country_code(&self) -> String {
        self.sub_region.country_code()
    }
    pub(crate) fn region(&self) -> String {
        self.sub_region.region()
    }
    pub(crate) fn region_code(&self) -> String {
        self.sub_region.region_code()
    }
    pub(crate) fn sub_region(&self) -> String {
        self.sub_region.sub_region.to_owned()
    }
    pub(crate) fn sub_region_code(&self) -> String {
        self.sub_region.sub_region_code.to_owned()
    }

}
