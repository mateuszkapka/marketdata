mod data;
mod parsers;
mod writers;
mod symbology;
mod paths;
mod aggregates;
mod readers;

use core::panic;

use chrono::NaiveDate;
use clap::{arg, command};
use parsers::parser::*;
use writers::*;

use crate::{base_writer::BaseWriter, paths::scratch::{get_normalised_path, get_symbology_path}, parquet_writer::ParquetWriter, schemas::get_symbology_schena, symbology::symbology_service::generate_symbology};

fn main() {
    
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
    let symbology = generate_symbology(&get_normalised_path(&date, &source))
                .unwrap_or_else(|err| panic!("Unable to load symbology: {}", err));

    let mut writer: Box<ParquetWriter> = Box::new(ParquetWriter::new(
        get_symbology_path(&date, &source),
        get_symbology_schena()
    ));

    writer.write_symbology(symbology);
    writer.finalize();
    

    println!("Symbology finished!");
}
