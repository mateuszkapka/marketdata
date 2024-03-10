use chrono::NaiveDate;

use crate::parsers::parser::ParserType;


// WSE
pub const WSE_TICK_FILES_PATH: &str = "/scratch/data/ticks/WSE";


// NASDAQ
#[allow(dead_code)]
pub const NASDAQ_TICK_FILES_PATH: &str = "/scratch/data/ticks/NASDAQ";
pub const NASDAQ_PATH_TO_QUOTES: &str = "/scratch/data/ticks/NASDAQ/xnas-itch-20240122.mbp-1.dbn.zst";
pub const NASDAQ_PATH_TO_TRADES: &str = "/scratch/data/ticks/NASDAQ/xnas-itch-20240122.trades.dbn.zst";
pub const NASDAQ_PATH_TO_SYMBOLOGY: &str = "/scratch/data/ticks/NASDAQ/symbology.json";


pub const NORMALISED_OUTPUT_PATH: &str = "/scratch/normalised_data/";
pub const SYMBOLOGY_OUTPUT_PATH: &str = "/scratch/symbology_data/";




// Output 
pub fn get_normalised_path(date: &NaiveDate, parser_type: &ParserType) -> String{
    format!("{}/{}_{}.parquet", NORMALISED_OUTPUT_PATH, date.format("%Y%m%d"), parser_type)
}

#[allow(dead_code)]
pub fn get_symbology_path(date: &NaiveDate, parser_type: &ParserType) -> String {
    format!("{}/{}_{}.parquet", SYMBOLOGY_OUTPUT_PATH, date.format("%Y%m%d"), parser_type)
}
