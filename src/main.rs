mod data;
mod parsers;

use chrono::NaiveDate;
use parsers::parser::*;

fn main() {
    let parser = parsers::parser::Parser{};
    let date = NaiveDate::from_ymd_opt(2024, 01, 22).unwrap();
    let _result = parser.parse_market_data(&date, ParserType::WSE);
    println!("Hello, world!");
}
