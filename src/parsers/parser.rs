use std::collections::HashMap;

use chrono::NaiveDate;
use crate::data::symbol::*;
use crate::parsers::wse_parser::{self};

pub enum ParserType{
    WSE
}

pub struct Parser{

}

impl Parser{
    pub fn parse_market_data(&self, date: &NaiveDate, parser_type: ParserType) -> HashMap<String,Symbol> {
        match parser_type{
            ParserType::WSE => {
                let parser = wse_parser::WSEParser::new();
                parser.parse_market_data(date)
            }
        }

    }
}