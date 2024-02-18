mod data;
mod parsers;
mod writers;

use chrono::NaiveDate;
use parsers::parser::*;
use writers::*;

use crate::{
    base_writer::BaseWriter, data::event_header::EventHeader, parquet_writer::ParquetWriter,
};
use std::{collections::HashSet, process::exit, str::FromStr};

fn main() {
    let parser = parsers::parser::Parser {};
    let date = NaiveDate::from_ymd_opt(2024, 01, 22).unwrap();

    let cmd = clap::Command::new("raw")
    .arg(
        clap::Arg::new("source")
            .value_parser(clap::builder::PossibleValuesParser::new(["WSE", "NASDAQ"]))
            .required(true)
    );
    
    let matches = cmd.get_matches();
    let source_str: String = match matches.get_one::<String>("source") {
        Some(m) => m.clone(),
        None =>{
            exit(0);
            "".to_string()
        }
    };
    let source = ParserType::from_str(&source_str).expect("Invalid value for argument source!");


    let result = parser.parse_market_data(&date, source);

    let mut symbols: HashSet<String> = HashSet::new();
    for event in result {
        symbols.insert(event.get_symbol().to_string());
    }

    let writer = ParquetWriter {};
    writer.write_symbology(&symbols, &format!("normalised_data/{}_symbology.parquet", source_str));

    println!("Symbology running!");
}
