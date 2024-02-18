use std::cmp::max;
use std::ptr::eq;

use crate::data::quote::*;
use crate::data::trade::*;
use chrono::NaiveDateTime;

use crate::data::event_header::*;

#[derive(Debug)]
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

impl PartialEq for Event{
    fn eq(&self, other: &Self) -> bool {
        self.get_timestamp().eq(&other.get_timestamp())
    }
}

impl PartialOrd for Event {
    fn ge(&self, other: &Self) -> bool {
        self.get_timestamp().ge(&other.get_timestamp())
    }

    fn gt(&self, other: &Self) -> bool {
        self.get_timestamp().gt(&other.get_timestamp())
    }

    fn le(&self, other: &Self) -> bool {
        self.get_timestamp().le(&other.get_timestamp())
    }

    fn lt(&self, other: &Self) -> bool {
        self.get_timestamp().lt(&other.get_timestamp())
    }

    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.get_timestamp().partial_cmp(&other.get_timestamp())
    }
}

impl Eq for Event{
    fn assert_receiver_is_total_eq(&self) {
        self.get_timestamp().assert_receiver_is_total_eq()
    }
}

impl Ord for Event{
    fn clamp(self, min: Self, max: Self) -> Self
        where
            Self: Sized,
            Self: PartialOrd, {
                self
    }

    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.get_timestamp().cmp(&other.get_timestamp())
    }

    fn max(self, other: Self) -> Self
        where
            Self: Sized, {
                if self.get_timestamp() > other.get_timestamp(){
                    self
                }
                else{
                    other
                }
                    
    }

    fn min(self, other: Self) -> Self
        where
            Self: Sized, {
                if self.get_timestamp() <= other.get_timestamp(){
                    self
                }
                else{
                    other
                }
    }
}