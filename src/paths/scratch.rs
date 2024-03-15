use chrono::NaiveDate;

use crate::parsers::parser::ParserType;


// WSE
pub const WSE_TICK_FILES_PATH: &str = "/scratch/data/ticks/WSE";


// NASDAQ
#[allow(dead_code)]
pub const NASDAQ_TICK_FILES_PATH: &str = "/scratch/data/ticks/NASDAQ";
pub const NASDAQ_PATH_TO_SYMBOLOGY: &str = "/scratch/data/ticks/NASDAQ/symbology.json";


pub const NORMALISED_OUTPUT_PATH: &str = "/scratch/normalised_data/";
pub const SYMBOLOGY_OUTPUT_PATH: &str = "/scratch/symbology_data/";
pub const AGGREGATES_OUTPUT_PATH: &str = "/scratch/aggregate_data/";


pub fn get_nasdaq_path_to_quotes(date: &NaiveDate) -> String{
    format!("/scratch/data/ticks/NASDAQ/xnas-itch-{}.mbp-1.dbn.zst", date.format("%Y%m%d"))
}

pub fn get_nasdaq_path_to_trades(date: &NaiveDate) -> String{
    format!("/scratch/data/ticks/NASDAQ/xnas-itch-{}.trades.dbn.zst", date.format("%Y%m%d"))
}

// Output 
pub fn get_normalised_path(date: &NaiveDate, parser_type: &ParserType) -> String{
    format!("{}/{}_{}.parquet", NORMALISED_OUTPUT_PATH, date.format("%Y%m%d"), parser_type)
}

#[allow(dead_code)]
pub fn get_symbology_path(date: &NaiveDate, parser_type: &ParserType) -> String {
    format!("{}/{}_{}.parquet", SYMBOLOGY_OUTPUT_PATH, date.format("%Y%m%d"), parser_type)
}

#[allow(dead_code)]
pub fn get_aggregates_path(date: &NaiveDate, parser_type: &ParserType) -> String {
    format!("{}/{}_{}.parquet", AGGREGATES_OUTPUT_PATH, date.format("%Y%m%d"), parser_type)
}
