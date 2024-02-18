mod data;
mod parsers;
mod writers;

use std::{rc::Rc, str::FromStr};

use chrono::NaiveDate;
use parsers::parser::*;
use writers::{base_writer::BaseWriter, *};

use crate::parquet_writer::ParquetWriter;

 fn main() {
    let cmd = clap::Command::new("raw")
    .arg(
        clap::Arg::new("source")
            .value_parser(clap::builder::PossibleValuesParser::new(["WSE", "NASDAQ"]))
            .required(true)
    );
    let matches = cmd.try_get_matches().unwrap();
    let date = NaiveDate::from_ymd_opt(2024, 01, 22).unwrap();
    let source_str: String = matches.get_one::<String>("source").unwrap().clone();
    let source = ParserType::from_str(&source_str).expect("Invalid value for argument source!");

    let parser = parsers::parser::Parser {};
    let mut writer: Box<dyn BaseWriter> = Box::new(ParquetWriter::new(
        format!("normalised_data/{}.parquet", &source_str).to_string()
    ));

    let result = parser.parse_market_data(&date, source, &mut writer);
    drop(writer)
}
