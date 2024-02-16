

use crate::data::event::Event;

use crate::parsers::wse_parser::{self};
use chrono::NaiveDate;

use super::databento_parser;

#[allow(dead_code)]
pub enum ParserType {
    WSE,
    NASDAQ
}

pub struct Parser {}

impl Parser {
    pub fn parse_market_data(&self, date: &NaiveDate, parser_type: ParserType) -> Vec<Event> {
        match parser_type {
            ParserType::WSE => {
                let parser = wse_parser::WSEParser::new();
                parser.parse_market_data(date)
            },
            ParserType::NASDAQ => {
                let parser = databento_parser::NasdaqParser::new();
                parser.parse_market_data(date)
            }
        }
    }
}
