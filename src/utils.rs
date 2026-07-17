use polars::prelude::{CsvWriter, DataFrame, SerReader, SerWriter};
use std::fs::File;
use std::time::Instant;

pub fn print_elapsed(start: &Instant, label: &str) {
    let millis = start.elapsed().as_millis();
    let seconds = millis / 1000;
    let (h, m, s) = (seconds / 3600, (seconds % 3600) / 60, seconds % 60);
    println!("{}: {:02}:{:02}:{:02}.{:03}", label, h, m, s, millis % 1000);
}

pub fn write_csv(df: &mut DataFrame, filename: &str) -> crate::error::Result<()> {
    let file = File::create(filename)?;
    CsvWriter::new(&file)
        .include_header(true)
        .finish(df)
        .map_err(Into::into)
}

pub fn load_csv(path: &str) -> crate::error::Result<DataFrame> {
    polars::prelude::CsvReadOptions::default()
        .with_has_header(true)
        .try_into_reader_with_file_path(Some(path.into()))?
        .finish()
        .map_err(Into::into)
}