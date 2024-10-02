use polars::prelude::{DataFrame, LazyCsvReader, LazyFileListReader};

/**
Loads the csv consisting of all locations for which data is to be scraped.
*/

pub(super) fn load_data(loc_file: &str) -> DataFrame {
    LazyCsvReader::new(loc_file)
        .with_has_header(true)
        .finish()
        .and_then(|f| f.collect())
        .unwrap_or_else(|e| panic!("Failed to load {}: {:?}", loc_file, e))
}
