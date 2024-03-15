use std::collections::HashSet;

use simple_error::SimpleError;

use crate::data::event_header::EventHeader;

use crate::readers::parquet_reader::ParquetStreamReader;

pub fn generate_symbology(filename: &str) -> Result<HashSet<String>, SimpleError>{
    let mut result: HashSet<String> = HashSet::new();
    let mut reader:ParquetStreamReader<_>  = ParquetStreamReader{
        filter: None,
        on_event: |event| {
            result.insert(event.get_symbol().to_string());
        }
    };

    reader.read_market_data(filename)?;
    Ok(result)
}