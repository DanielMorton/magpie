use polars::prelude::{CsvWriter, DataFrame, SerWriter};
use std::error::Error;
use std::fs::File;
use std::time::Instant;

/// Strips a single matching pair of surrounding quote characters (`"` or `'`) from `value`,
/// if present. Values that aren't quoted (or use mismatched quote characters) are returned
/// unchanged, so this never silently truncates legitimate, unquoted data.
pub(super) fn remove_quote(value: &str) -> String {
    let trimmed = value.trim();
    let bytes = trimmed.as_bytes();

    if let (Some(&first), Some(&last)) = (bytes.first(), bytes.last()) {
        let is_quote = |b: u8| b == b'"' || b == b'\'';
        if bytes.len() >= 2 && first == last && is_quote(first) {
            return trimmed[1..trimmed.len() - 1].to_string();
        }
    }

    trimmed.to_string()
}

/// Prints the run time of a procedure in human readable format.
pub fn print_hms(start: &Instant) {
    let millis = start.elapsed().as_millis();
    let seconds = millis / 1000;
    let (hour, minute, second) = (seconds / 3600, (seconds % 3600) / 60, seconds % 60);
    println!(
        "Elapsed time: {:02}:{:02}:{:02}.{:03}",
        hour,
        minute,
        second,
        millis % 1000
    );
}

pub fn write_csv(df: &mut DataFrame, filename: &str) -> Result<(), Box<dyn Error>> {
    let file = File::create(filename)?;
    CsvWriter::new(&file)
        .include_header(true)
        .finish(df)
        .map_err(|e| e.into())
}

#[cfg(test)]
mod tests {
    use super::remove_quote;

    #[test]
    fn strips_matching_double_quotes() {
        assert_eq!(remove_quote("\"California\""), "California");
    }

    #[test]
    fn strips_matching_single_quotes() {
        assert_eq!(remove_quote("'California'"), "California");
    }

    #[test]
    fn leaves_unquoted_values_untouched() {
        assert_eq!(remove_quote("California"), "California");
    }

    #[test]
    fn leaves_mismatched_quotes_untouched() {
        assert_eq!(remove_quote("\"California'"), "\"California'");
    }

    #[test]
    fn leaves_single_char_untouched() {
        assert_eq!(remove_quote("A"), "A");
    }

    #[test]
    fn leaves_empty_string_untouched() {
        assert_eq!(remove_quote(""), "");
    }
}
