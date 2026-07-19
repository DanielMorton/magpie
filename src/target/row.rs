#[derive(Debug, Clone)]
pub struct LocationRow {
    pub country: String,
    pub region: String,
    pub sub_region: String,
    pub hotspot: Option<String>,
}

impl LocationRow {
    pub fn new_hotspot(
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

    pub fn new_location(country: String, region: String, sub_region: String) -> Self {
        Self {
            country,
            region,
            sub_region,
            hotspot: None,
        }
    }
}
