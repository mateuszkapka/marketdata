mod data;
mod parsers;
mod writers;
mod symbology;
mod paths;
mod aggregates;
mod readers;

use core::panic;

use chrono::NaiveDate;
use parsers::parser::*;
use writers::*;

use crate::{base_writer::BaseWriter, paths::scratch::{get_normalised_path, get_symbology_path}, parquet_writer::ParquetWriter, schemas::get_symbology_schena, symbology::symbology_service::generate_symbology};

fn main() {
    
    let date = NaiveDate::from_ymd_opt(2024, 01, 22).unwrap();

    let cmd = clap::Command::new("raw")
    .arg(
        clap::Arg::new("source")
            .value_parser(clap::builder::EnumValueParser::<ParserType>::new())
            .required(true)
    );
    
    let matches = cmd.get_matches();
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
