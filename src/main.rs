mod data;
mod parsers;
mod writers;
use log::info;
use simple_logger::{SimpleLogger, set_up_color_terminal};

use std::str::FromStr;

use chrono::NaiveDate;
use parsers::parser::*;
use writers::{base_writer::BaseWriter, *};

use crate::{common::{get_market_data_schema, get_normalised_path}, parquet_writer::ParquetWriter};

 fn main() {
    set_up_color_terminal();
    SimpleLogger::new().init().unwrap();

    let cmd = clap::Command::new("raw")
    .arg(
        clap::Arg::new("source")
            .value_parser(clap::builder::PossibleValuesParser::new(["WSE", "NASDAQ"]))
            .required(true)
    );
    info!("Starting...");
    let matches = cmd.try_get_matches().unwrap();
    let date = NaiveDate::from_ymd_opt(2024, 01, 22).unwrap();
    let source_str: String = matches.get_one::<String>("source").unwrap().clone();
    let source = ParserType::from_str(&source_str).expect("Invalid value for argument source!");

    let parser = parsers::parser::Parser {};
    let mut writer: Box<ParquetWriter> = Box::new(ParquetWriter::new(
        get_normalised_path(&date, &source),
        get_market_data_schema()
    ));

    parser.parse_market_data(&date, source, &mut writer);
    writer.finalize();
}
