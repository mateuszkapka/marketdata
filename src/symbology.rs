mod data;
mod parsers;
mod writers;

use chrono::NaiveDate;
use parsers::parser::*;
use writers::*;

use crate::parquet_writer::ParquetWriter;

use std::{process::exit, rc::Rc, str::FromStr};

fn main() {
    
    let _date = NaiveDate::from_ymd_opt(2024, 01, 22).unwrap();

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
        }
    };
    let _source = ParserType::from_str(&source_str).expect("Invalid value for argument source!");
    let _parser = parsers::parser::Parser {};
    let _writer = Rc::new(ParquetWriter::new(
        format!("normalised_data/{}_symbology.parquet", &source_str).to_string()
    ));

    //let result = parser.parse_market_data(&date, source);

    println!("Symbology running!");
}
