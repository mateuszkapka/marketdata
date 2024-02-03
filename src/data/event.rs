use crate::data::trade::*;
use crate::data::quote::*;
use chrono::NaiveDateTime;

use crate::data::event_header::*;

pub enum Event{
    Trade(Trade),
    Quote(Quote)
}

impl EventHeader for Event{
    fn get_timestamp(&self) -> NaiveDateTime{
        match self {
            Event::Trade(trade) => NaiveDateTime::new(trade.trade_date, trade.trade_time),
            Event::Quote(quote) => NaiveDateTime::new(quote.quote_date, quote.quote_time)
        }
        
    }
}