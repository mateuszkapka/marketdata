mod data;
mod aggregates;
mod writers;
mod parsers;
mod readers;
mod paths;

use aggregates::{aggregate_framework::AggregateFramework, test_aggregates::VolumeAggregate};
use parsers::parser::ParserType;
use paths::scratch::get_aggregates_path;
use readers::filters::{NoOpFilter, ParquetFilter, SymbolFilter};
use writers::{parquet_writer::ParquetWriter, schemas::get_aggregates_schema};
use std::process::exit;
use chrono::NaiveDate;
use crate::writers::base_writer::BaseWriter;


fn main() {
    let date = NaiveDate::from_ymd_opt(2024, 01, 22).unwrap();

    let cmd = clap::Command::new("raw")
    .arg(
        clap::Arg::new("source")
            .value_parser(clap::builder::EnumValueParser::<ParserType>::new())
            .required(true)
    ).arg(
        clap::Arg::new("symbol")
            .required(false)
    );
    
    let matches = cmd.get_matches();
    let source  = matches.get_one::<ParserType>("source").unwrap();

    let filter = matches.get_one::<String>("symbol");

    if filter.is_none(){
        let framework = AggregateFramework::new(&source, &date, NoOpFilter{});
        run_agg_framework(framework, &source, &date);
    } else{
        let framework = AggregateFramework::new(&source, &date, SymbolFilter::new(filter.unwrap()));
        run_agg_framework(framework, &source, &date);
    }
    

    exit(0);
}

fn run_agg_framework<T: ParquetFilter + Clone>(mut framework: AggregateFramework<T>, source: &ParserType, date: &NaiveDate){
    framework.register_aggregate::<VolumeAggregate>();
    let result = framework.run();

    let mut writer: Box<ParquetWriter> = Box::new(ParquetWriter::new(
        get_aggregates_path(&date, &source),
        get_aggregates_schema()
    ));

    writer.write_aggregates(result);
    writer.finalize();
}