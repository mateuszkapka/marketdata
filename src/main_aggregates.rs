mod data;
mod aggregates;
mod writers;
mod parsers;
mod readers;
mod paths;

use aggregates::aggregate_framework::{register_default_aggregates, AggregateFramework};
use clap::{arg, command};
use parsers::parser::ParserType;
use paths::scratch::get_aggregates_path;
use readers::filters::SymbolFilter;
use writers::{parquet_writer::ParquetWriter, schemas::get_aggregates_schema};
use std::process::exit;
use chrono::NaiveDate;
use crate::writers::base_writer::BaseWriter;


fn main() {
    let matches = command!() // requires `cargo` feature
    .arg(arg!([source] "Surce")
        .value_parser(clap::builder::EnumValueParser::<ParserType>::new()))
    .arg(arg!(
        -s --symbol <SYMBOL> "Filter by symbol"
    ).required(false))
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
        

    let filter_str = matches.get_one::<String>("symbol");
    let filter = filter_str.map_or_else(|| None, |x| Some(SymbolFilter::new(&x)));
    let source  = matches.get_one::<ParserType>("source").unwrap();

    let framework = AggregateFramework::new(&source, &date, filter);
    run_agg_framework(framework, &source, &date);
    

    exit(0);
}

fn run_agg_framework(mut framework: AggregateFramework, source: &ParserType, date: &NaiveDate){
    register_default_aggregates(&mut framework)
        .unwrap_or_else(|err| panic!("Aggregate registration failed: {}", err));
    let result = framework.run()
        .unwrap_or_else(|err| panic!("Calculating aggregates failed: {}", err));

    let mut writer: Box<ParquetWriter> = Box::new(ParquetWriter::new(
        get_aggregates_path(&date, &source),
        get_aggregates_schema()
    ));

    writer.write_aggregates(result);
    writer.finalize();
}