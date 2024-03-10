use chrono::NaiveDate;

use crate::parsers::parser::ParserType;


// WSE
pub const WSE_TICK_FILES_PATH: &str = "/scratch/data/ticks/WSE";


// NASDAQ
pub const NASDAQ_PATH_TO_QUOTES: &str = "/scratch/data/ticks/NASDAQ/xnas-itch-20240122.mbp-1.dbn.zst";
pub const NASDAQ_PATH_TO_TRADES: &str = "/scratch/data/ticks/NASDAQ/xnas-itch-20240122.trades.dbn.zst";
pub const NASDAQ_PATH_TO_SYMBOLOGY: &str = "/scratch/data/ticks/NASDAQ/symbology.json";


// Output 
pub fn get_normalised_path(date: &NaiveDate, parser_type: &ParserType) -> String{
    format!("/scratch/normalised_data/{}_{}.parquet", date.format("%Y%m%d"), parser_type)
}

#[allow(dead_code)]
pub fn get_symbology_path(date: &NaiveDate, parser_type: &ParserType) -> String {
    format!("/scratch/symbology_data/{}_{}.parquet", date.format("%Y%m%d"), parser_type)
}
