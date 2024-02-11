use chrono::NaiveDate;

use crate::data::event::*;
use Vec;

pub struct Symbol {
    pub date: NaiveDate,
    pub symbol: String,

    pub events: Vec<Event>,
}

impl Symbol {
    pub fn new(date: &NaiveDate, ticker: &str) -> Self {
        Symbol {
            date: date.clone(),
            symbol: ticker.to_string(),
            events: Vec::new(),
        }
    }
}
