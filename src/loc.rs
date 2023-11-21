use polars::prelude::{DataFrame, LazyCsvReader, LazyFileListReader};

pub(super) fn load_data(loc_file: &str) -> DataFrame {
    match LazyCsvReader::new(loc_file)
        .has_header(true)
        .finish()
        .map(|f| f.collect())
    {
        Ok(r) => match r {
            Ok(region) => region,
            Err(e) => panic!("Failed to load {}:\n {}", loc_file, e),
        },
        Err(e) => panic!("Failed to load {}:\n {}", loc_file, e),
    }
}
