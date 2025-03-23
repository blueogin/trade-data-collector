use std::error::Error;
use std::fs::File;

use crate::utils::OrderEvent;
use csv::{ReaderBuilder, Writer};

use crate::constants;

/// Initializes a CSV file with headers
pub fn initialize_csv(filename: &str) -> Result<(), Box<dyn Error>> {
    let mut writer = Writer::from_writer(
        File::options()
            .write(true) // Open file for writing
            .create(true) // Create the file if it doesn't exist
            .truncate(true) // Truncate the file to zero length if it exists
            .open(filename)?, // Open the file
    );

    // Write headers
    writer.write_record(constants::CSV_HEADER)?;
    writer.flush()?;

    Ok(())
}

/// Writes order events to a CSV file.
pub fn write_to_csv(filename: &str, events: &[OrderEvent]) -> Result<(), Box<dyn Error>> {
    let mut writer = Writer::from_writer(File::options().append(true).open(filename)?);

    for event in events {
        writer.write_record(&[
            format!("{:?}", event.tx_origin),
            event.event_type.clone(),
            format!("{:?}", event.txn_hash),
            event.timestamp.to_string(),
        ])?;
    }

    writer.flush()?;
    Ok(())
}

pub fn verify_csv(filename: &str, expected_row_count: usize) -> bool {
    // Open the CSV file
    let file = match File::open(filename) {
        Ok(file) => file,
        Err(_) => return false,
    };

    // Create a CSV reader
    let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);

    // Read the header
    let headers = match rdr.headers() {
        Ok(headers) => headers,
        Err(_) => return false,
    };

    // Convert CSV_HEADER to a comma-separated string and compare with the file header
    let expected_header = constants::CSV_HEADER.join(",");
    let file_header = headers.iter().collect::<Vec<&str>>().join(",");

    if file_header != expected_header {
        return false;
    }

    // Count the number of non-empty data rows using an iterator chain
    let line_count = rdr
        .records()
        .filter_map(|record| record.ok()) // Unwrap the Result and only keep valid records
        .filter(|record| !record.is_empty()) // Exclude empty lines
        .count(); // Get the count of valid (non-empty) rows

    // Ensure the file contains the expected number of lines (excluding header)
    if line_count != expected_row_count {
        return false;
    }

    true
}
