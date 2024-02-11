mod data;
mod parsers;
mod writers;

use chrono::NaiveDate;
use parsers::parser::*;
use writers::*;

use crate::{base_writer::BaseWriter, parquet_writer::ParquetWriter};

fn main() {
    let parser = parsers::parser::Parser {};
    let date = NaiveDate::from_ymd_opt(2024, 01, 22).unwrap();
    let result = parser.parse_market_data(&date, ParserType::WSE);

    let writer = ParquetWriter {};
    writer.write_matket_data(&result, "WSE_marketdata.parquet");

    println!("Hello, world!");
}
