mod data;
mod parsers;
mod writers;

use chrono::NaiveDate;
use parsers::parser::*;
use writers::*;

use crate::{
    base_writer::BaseWriter, data::event_header::EventHeader, parquet_writer::ParquetWriter,
};
use std::collections::HashSet;

fn main() {
    let parser = parsers::parser::Parser {};
    let date = NaiveDate::from_ymd_opt(2024, 01, 22).unwrap();
    let result = parser.parse_market_data(&date, ParserType::WSE);

    let mut symbols: HashSet<String> = HashSet::new();
    for event in result {
        symbols.insert(event.get_symbol().to_string());
    }

    let writer = ParquetWriter {};
    writer.write_symbology(&symbols, "WSE_symbology.parquet");

    println!("Symbology running!");
}
