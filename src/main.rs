mod data;
mod parsers;

use chrono::NaiveDate;
use parsers::parser::*;

use crate::data::event::Event;


fn main() {
    let parser = parsers::parser::Parser{};
    let date = NaiveDate::from_ymd_opt(2024, 01, 22).unwrap();
    let result = parser.parse_market_data(&date, ParserType::WSE);

    for (key,value) in result{
        println!("Symbol: {}", key);
        for event in value.events{
            match &event {
                Event::Trade(trade) => println!("{} : T", trade.trade_time),
                Event::Quote(quote) => println!("{} : Q", quote.quote_time),

            }
        }
    }

    println!("Hello, world!");
}
