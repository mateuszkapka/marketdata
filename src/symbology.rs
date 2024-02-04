mod data;
mod parsers;
mod writers;

use chrono::NaiveDate;
use parsers::parser::*;
use writers::*;

use crate::{base_writer::BaseWriter, parquet_writer::ParquetWriter};

fn main() {
    let parser = parsers::parser::Parser{};
    let date = NaiveDate::from_ymd_opt(2024, 01, 22).unwrap();
    let result = parser.parse_market_data(&date, ParserType::WSE);

    let mut symbols: Vec<String> = Vec::new();
    for (key,_value) in result{
        symbols.push(key.clone());
    }

    let writer = ParquetWriter{};
    writer.write_symbology(&symbols, "WSE_symbology.parquet");

    println!("Symbology running!");
}