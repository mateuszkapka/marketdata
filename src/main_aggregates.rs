mod data;
mod writers;
mod parsers;
mod readers;
mod aggregates;

use aggregates::aggregate_framework::AggregateFramework;
use parsers::parser::ParserType;
use aggregates::test_aggregates::SimpleAggregate;
use std::process::exit;
use chrono::NaiveDate;
use writers::*;

use std::str::FromStr;


fn main() {
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
        }
    };
    let source = ParserType::from_str(&source_str).expect("Invalid value for argument source!");
    let mut framework = AggregateFramework::new(&source, &date);
    framework.register_aggregate::<SimpleAggregate>();
    framework.run();

    exit(0);
}