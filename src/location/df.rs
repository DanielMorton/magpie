use crate::error::Result;
use crate::location::loc::{Hotspot, SubRegion};

pub fn write_sub_regions_csv(sub_regions: &[SubRegion], filename: &str) -> Result<()> {
    let mut writer = csv::Writer::from_path(filename)?;
    writer.write_record([
        "country",
        "country_code",
        "region",
        "region_code",
        "sub_region",
        "sub_region_code",
    ])?;

    for sr in sub_regions {
        writer.write_record([
            sr.country(),
            sr.country_code(),
            sr.region(),
            sr.region_code(),
            sr.name(),
            sr.code(),
        ])?;
    }

    writer.flush()?;
    Ok(())
}

pub fn write_hotspots_csv(hotspots: &[Hotspot], filename: &str) -> Result<()> {
    let mut writer = csv::Writer::from_path(filename)?;
    writer.write_record([
        "country",
        "country_code",
        "region",
        "region_code",
        "sub_region",
        "sub_region_code",
        "hotspot",
        "hotspot_code",
    ])?;

    for h in hotspots {
        writer.write_record([
            h.country(),
            h.country_code(),
            h.region(),
            h.region_code(),
            h.sub_region(),
            h.sub_region_code(),
            h.name(),
            h.code(),
        ])?;
    }

    writer.flush()?;
    Ok(())
}
