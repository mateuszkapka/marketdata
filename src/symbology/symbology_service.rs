use std::collections::HashSet;

use crate::data::event_header::EventHeader;

use crate::readers::parquet_reader::ParquetStreamReader;

pub fn generate_symbology(filename: &str) -> HashSet<String>{
    let mut result: HashSet<String> = HashSet::new();
    let mut reader = ParquetStreamReader{
        on_event: |event| {
            result.insert(event.get_symbol().to_string());
        }
    };

    reader.read_market_data(filename);
    result
}