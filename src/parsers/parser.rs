use core::fmt;
use std::str::FromStr;


use crate::writers::parquet_writer::ParquetWriter;
use crate::parsers::wse_parser::{self};
use chrono::NaiveDate;
use clap::ValueEnum;

use super::databento_parser;

#[allow(dead_code)]
#[derive(Debug, ValueEnum, Clone)]
pub enum ParserType {
    WSE,
    NASDAQ
}

impl FromStr for ParserType {
    type Err = ();

    fn from_str(input: &str) -> Result<ParserType, Self::Err> {
        match input {
            "WSE"  => Ok(ParserType::WSE),
            "NASDAQ"  => Ok(ParserType::NASDAQ),
            _      => Err(()),
        }
    }
}

impl fmt::Display for ParserType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct Parser {}

impl Parser {
    pub fn parse_market_data(&self, date: &NaiveDate, parser_type: ParserType, writer: & mut Box<ParquetWriter>, symbol_filter: Option<&String>){
        match parser_type {
            ParserType::WSE => {
                let mut parser = wse_parser::WSEParser::new(writer);
                parser.parse_market_data(date, symbol_filter)
            },
            ParserType::NASDAQ => {
                let mut parser = databento_parser::NasdaqParser::new(writer, date);
                parser.parse_market_data(date, symbol_filter)
            }
        }
    }
}
