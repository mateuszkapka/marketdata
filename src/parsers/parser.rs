use std::collections::HashMap;

use crate::data::event::Event;
use crate::data::symbol::*;
use crate::parsers::wse_parser::{self};
use chrono::NaiveDate;

pub enum ParserType {
    WSE,
}

pub struct Parser {}

impl Parser {
    pub fn parse_market_data(&self, date: &NaiveDate, parser_type: ParserType) -> Vec<Event> {
        match parser_type {
            ParserType::WSE => {
                let parser = wse_parser::WSEParser::new();
                parser.parse_market_data(date)
            }
        }
    }
}
