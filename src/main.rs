#[allow(dead_code)]
mod data;
mod parsers;
mod writers;
mod readers;
mod aggregates;
mod paths;
mod utils;
use clap::command;
use simple_logger::{SimpleLogger, set_up_color_terminal};

use chrono::NaiveDate;
use parsers::parser::*;
use writers::{base_writer::BaseWriter, *};

use crate::{paths::scratch::get_normalised_path, parquet_writer::ParquetWriter, schemas::get_market_data_schema};

use clap::arg;

 fn main() {
    set_up_color_terminal();
    SimpleLogger::new().init().unwrap();

    let matches = command!() // requires `cargo` feature
    .arg(arg!([source] "Surce")
        .value_parser(clap::builder::EnumValueParser::<ParserType>::new()))
    .arg(
        arg!(
            -d --date <DATE> "Date to run the aggregator on"
        )
        // We don't have syntax yet for optional options, so manually calling `required`
        .required(false)
    )
    .get_matches();

    let date: NaiveDate = NaiveDate::parse_from_str(matches.get_one::<String>("date").unwrap(), "%Y%m%d")
            .unwrap_or_else(|err| panic!("Invalid parameter date: {}", err));
    let source  = matches.get_one::<ParserType>("source").unwrap();

    let parser = parsers::parser::Parser {};
    let mut writer: Box<ParquetWriter> = Box::new(ParquetWriter::new(
        get_normalised_path(&date, &source),
        get_market_data_schema()
    ));

    parser.parse_market_data(&date, source.clone(), &mut writer);
    writer.finalize();
}
