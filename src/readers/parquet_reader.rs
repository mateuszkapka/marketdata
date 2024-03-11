

use std::fs::File;

use crate::{writers::schemas::{get_market_data_schema, get_symbology_schena, map_columns_to_indexes}, data::event::Event};

use chrono::NaiveDateTime;
use parquet::{file::{reader::FileReader, serialized_reader::SerializedFileReader}, record::RowAccessor};

use super::filters::ParquetFilter;

pub struct ParquetReader
{}
pub struct ParquetStreamReader<TFilter, F>
where
    F: FnMut(Event),
    TFilter: ParquetFilter
{
    pub on_event: F,
    pub filter: TFilter
}

impl<TFilter, F> ParquetStreamReader<TFilter, F>
where
    F: FnMut(Event),
    TFilter: ParquetFilter
{

    #[allow(deprecated)]
    pub fn read_market_data(&mut self, filename: &str) {
        let schema = get_market_data_schema();
        let colums_mapping = map_columns_to_indexes(&schema);

        // Open the Parquet file
        let file = File::open(filename).unwrap();
        let reader = SerializedFileReader::new(file).unwrap();
        let mut arrow_reader = reader.get_row_iter(None).unwrap();

        // Iterate over rows and extract the "symbol" column
        while let Some(record_result) = arrow_reader.next() {
            let record = record_result.unwrap();
            let event_type= record.get_string(colums_mapping.get("type").unwrap().clone()).unwrap().clone(); 
            if self.filter.should_filter_row(&record, &colums_mapping){
                continue;
            }

            match event_type.as_str(){
                 "Trade" => {
                    (self.on_event)(Event::Trade(crate::data::trade::Trade {
                         symbol: record.get_string(colums_mapping.get("symbol").unwrap().clone()).unwrap().clone(),
                         trade_timestamp: NaiveDateTime::from_timestamp_millis(record.get_timestamp_millis(colums_mapping.get("timestamp").unwrap().clone()).unwrap().clone()).unwrap(), 
                         exchange_date:  "".to_string(),
                         price: record.get_double(colums_mapping.get("trade_price").unwrap().clone()).unwrap().clone(),
                         volume: record.get_long(colums_mapping.get("trade_volume").unwrap().clone()).unwrap().clone()
                         }))},
                 "Quote" =>{
                    (self.on_event)(Event::Quote(crate::data::quote::Quote {
                        symbol: record.get_string(colums_mapping.get("symbol").unwrap().clone()).unwrap().clone(),
                        quote_date: NaiveDateTime::from_timestamp_millis(record.get_timestamp_millis(colums_mapping.get("timestamp").unwrap().clone()).unwrap().clone()).unwrap(), 
                        exchange_date: "".to_string(),
                        ask_price: record.get_double(colums_mapping.get("ask_price").unwrap().clone()).unwrap().clone(),
                        bid_price: record.get_double(colums_mapping.get("bid_price").unwrap().clone()).unwrap().clone(),
                        ask_size: record.get_long(colums_mapping.get("ask_size").unwrap().clone()).unwrap().clone(),
                        bid_size: record.get_long(colums_mapping.get("bid_size").unwrap().clone()).unwrap().clone(),
                        market_period: record.get_string(colums_mapping.get("market_period").unwrap().clone()).unwrap().clone()
                        }))},
                 _ => panic!("Unable to parse event type {}", &event_type)
                 }
        }
    }

    
}

impl ParquetReader{
    pub fn read_symbology(&self, filename: &str) -> Vec<String>{
        let mut result = Vec::new();
        let schema = get_symbology_schena();
        let colums_mapping = map_columns_to_indexes(&schema);

        let file = File::open(filename).unwrap();
        let reader = SerializedFileReader::new(file).unwrap();
        let mut arrow_reader = reader.get_row_iter(None).unwrap();

        while let Some(record_result) = arrow_reader.next() {
            let record = record_result.unwrap();
            let column_index = colums_mapping.get("symbol").unwrap().clone();
            result.push(record.get_string(column_index).unwrap().to_string())
        }

        result
    }
}       
