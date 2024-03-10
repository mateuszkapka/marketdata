use clap::ValueEnum;
use cmd::{promote::{promote, PromoteTarget}, pull::pull};

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
        clap::Arg::new("target")
            .long("target")
            .required(false)
            .value_parser(clap::builder::EnumValueParser::<PromoteTarget>::new())
            .default_value("aggs")
    );

    let matches = cmd.get_matches();
    match matches.get_one::<CmdMode>("cmd").unwrap() {
        CmdMode::Pull => pull(),
        CmdMode::Promote => promote(matches.get_one::<PromoteTarget>("target").unwrap())
    }
    
}
