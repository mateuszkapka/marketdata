use std::{process::exit};

use clap::ValueEnum;
use cmd::{promote::PromoteTarget, pull::pull};
use parsers::parser::ParserType;

#[allow(dead_code)]
mod data;
mod parsers;
mod writers;
mod readers;
mod aggregates;
mod paths;
mod cmd;
mod utils;

#[derive(Clone, Debug, ValueEnum)]
enum CmdMode{
    Pull,
    Promote
}

fn main(){
    let cmd = clap::Command::new("raw")
    .arg(
        clap::Arg::new("cmd")
            .value_parser(clap::builder::EnumValueParser::<CmdMode>::new())
            .required(true)
    )
    .arg(
        clap::Arg::new("source")
            .long("market")
            .value_parser(clap::builder::EnumValueParser::<ParserType>::new())
            .required(false)
    );
    // .arg(
    //     clap::Arg::new("target")
    //         .long("target")
    //         .required(false)
    //        .default_value(PromoteTarget::Aggs)
     

    let matches = cmd.get_matches();
    match matches.get_one::<CmdMode>("cmd").unwrap() {
        CmdMode::Pull => pull(),
        CmdMode::Promote => {
            
        },
         v => {
            println!("command {:?} is not supported", v);
            exit(1);
        },
    }
    
}
