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
            Event::Trade(trade) => trade.trade_timestamp,
            Event::Quote(quote) => quote.quote_date
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
