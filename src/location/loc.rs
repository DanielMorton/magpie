#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Country {
    name: String,
    code: String,
}

impl Country {
    pub fn new(name: impl Into<String>, code: impl Into<String>) -> Self {
        Self { name: name.into(), code: code.into() }
    }
    pub fn name(&self) -> &str { &self.name }
    pub fn code(&self) -> &str { &self.code }
}

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct Region<'a> {
    name: String,
    code: String,
    country: &'a Country,
}

impl<'a> Region<'a> {
    pub fn new(name: impl Into<String>, code: impl Into<String>, country: &'a Country) -> Self {
        Self { name: name.into(), code: code.into(), country }
    }
    pub fn name(&self) -> &str { &self.name }
    pub fn code(&self) -> &str { &self.code }
    pub fn country(&self) -> &str { self.country.name() }
    pub fn country_code(&self) -> &str { self.country.code() }
}

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct SubRegion<'a> {
    name: String,
    code: String,
    region: &'a Region<'a>,
}

impl<'a> SubRegion<'a> {
    pub fn new(name: impl Into<String>, code: impl Into<String>, region: &'a Region<'a>) -> Self {
        Self { name: name.into(), code: code.into(), region }
    }
    pub fn name(&self) -> &str { &self.name }
    pub fn code(&self) -> &str { &self.code }
    pub fn region(&self) -> &str { self.region.name() }
    pub fn region_code(&self) -> &str { self.region.code() }
    pub fn country(&self) -> &str { self.region.country() }
    pub fn country_code(&self) -> &str { self.region.country_code() }
}

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct Hotspot<'a> {
    name: String,
    code: String,
    sub_region: &'a SubRegion<'a>,
}

impl<'a> Hotspot<'a> {
    pub fn new(name: impl Into<String>, code: impl Into<String>, sub_region: &'a SubRegion<'a>) -> Self {
        Self { name: name.into(), code: code.into(), sub_region }
    }
    pub fn name(&self) -> &str { &self.name }
    pub fn code(&self) -> &str { &self.code }
    pub fn sub_region(&self) -> &str { self.sub_region.name() }
    pub fn sub_region_code(&self) -> &str { self.sub_region.code() }
    pub fn region(&self) -> &str { self.sub_region.region() }
    pub fn region_code(&self) -> &str { self.sub_region.region_code() }
    pub fn country(&self) -> &str { self.sub_region.country() }
    pub fn country_code(&self) -> &str { self.sub_region.country_code() }
}