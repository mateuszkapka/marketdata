mod data;
mod parsers;
mod writers;

use std::str::FromStr;

use chrono::NaiveDate;
use parsers::parser::*;
use writers::*;

use crate::{base_writer::BaseWriter, parquet_writer::ParquetWriter};

 fn main() {
    let cmd = clap::Command::new("raw")
    .arg(
        clap::Arg::new("source")
            .value_parser(clap::builder::PossibleValuesParser::new(["WSE", "NASDAQ"]))
            .required(true)
    );
    let matches = cmd.try_get_matches().unwrap();

    let parser = parsers::parser::Parser {};
    let date = NaiveDate::from_ymd_opt(2024, 01, 22).unwrap();
    let source_str: String = matches.get_one::<String>("source").unwrap().clone();
    let source = ParserType::from_str(&source_str).expect("Invalid value for argument source!");
    let result = parser.parse_market_data(&date, source);

    let writer = ParquetWriter {};
    writer.write_matket_data(&result, &format!("normalised_data/{}.parquet", &source_str));

    println!("Hello, world!");
}
