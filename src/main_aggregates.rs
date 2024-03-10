mod data;
mod writers;
mod parsers;
mod readers;
mod aggregates;
mod paths;

use aggregates::{aggregate_framework::AggregateFramework, test_aggregates::VolumeAggregate};
use parsers::parser::ParserType;
use readers::filters::{NoOpFilter, ParquetFilter, SymbolFilter};
use std::process::exit;
use chrono::NaiveDate;

use std::str::FromStr;


fn main() {
    let date = NaiveDate::from_ymd_opt(2024, 01, 22).unwrap();

    let cmd = clap::Command::new("raw")
    .arg(
        clap::Arg::new("source")
            .value_parser(clap::builder::PossibleValuesParser::new(["WSE", "NASDAQ"]))
            .required(true)
    ).arg(
        clap::Arg::new("symbol")
            .required(false)
    );
    
    let matches = cmd.get_matches();
    let source_str: String = match matches.get_one::<String>("source") {
        Some(m) => m.clone(),
        None =>{
            exit(0);
        }
    };

    let source = ParserType::from_str(&source_str).expect("Invalid value for argument source!");
    let filter = matches.get_one::<String>("symbol");

    if filter.is_none(){
        let framework = AggregateFramework::new(&source, &date, NoOpFilter{});
        run_agg_framework(framework);
    } else{
        let framework = AggregateFramework::new(&source, &date, SymbolFilter::new(filter.unwrap()));
        run_agg_framework(framework);
    }
    

    exit(0);
}

fn run_agg_framework<T: ParquetFilter + Clone>(mut framework: AggregateFramework<T>){
    framework.register_aggregate::<VolumeAggregate>();
    let result = framework.run();

    for row in result{
        print!("{:?}", row);
    }
}