use crate::data::quote::*;
use crate::data::trade::*;
use chrono::NaiveDateTime;

use crate::data::event_header::*;

pub enum Event {
    Trade(Trade),
    Quote(Quote),
}

impl EventHeader for Event {
    fn get_timestamp(&self) -> NaiveDateTime {
        match self {
            Event::Trade(trade) => NaiveDateTime::new(trade.trade_date, trade.trade_time),
            Event::Quote(quote) => NaiveDateTime::new(quote.quote_date, quote.quote_time),
        }
    }

    fn get_type(&self) -> &str {
        match self {
            Event::Trade(_) => "Trade",
            Event::Quote(_) => "Quote",
        }
    }

    fn get_symbol(&self) -> &str {
        match self {
            Event::Trade(trade) => &trade.symbol,
            Event::Quote(quote) => &quote.symbol,
        }
    }
}
