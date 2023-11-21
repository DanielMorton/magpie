#[derive(Debug, Clone)]
pub struct LocationRow {
    country: String,
    region: String,
    sub_region: String,
    hotspot: Option<String>,
}
impl LocationRow {
    pub(crate) fn new(loc_vec: Vec<String>) -> Self {
        if loc_vec.len() == 3 {
            LocationRow::new_location(loc_vec[0].clone(), loc_vec[1].clone(), loc_vec[2].clone())
        } else {
            LocationRow::new_hotspot(
                loc_vec[0].clone(),
                loc_vec[1].clone(),
                loc_vec[2].clone(),
                loc_vec[3].clone(),
            )
        }
    }

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

    pub(crate) fn new_location(country: String, region: String, sub_region: String) -> Self {
        LocationRow {
            country,
            region,
            sub_region,
            hotspot: None,
        }
    }

    pub(crate) fn country(&self) -> String {
        self.country.clone()
    }
    pub(crate) fn region(&self) -> String {
        self.region.clone()
    }
    pub(crate) fn sub_region(&self) -> String {
        self.sub_region.clone()
    }
    pub(crate) fn hotspot(&self) -> Option<String> {
        self.hotspot.clone()
    }
}
pub struct SpeciesRow<'a> {
    common_name: &'a str,
    scientific_name: &'a str,
    percent: f32,
}

impl<'a> SpeciesRow<'a> {
    pub(crate) fn new(common_name: &'a str, scientific_name: &'a str, percent: f32) -> Self {
        SpeciesRow {
            common_name,
            scientific_name,
            percent,
        }
    }
    pub(crate) fn common_name(&self) -> &str {
        self.common_name
    }
    pub(crate) fn scientific_name(&self) -> &str {
        self.scientific_name
    }
    pub(crate) fn percent(&self) -> f32 {
        self.percent
    }
}
