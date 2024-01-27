use chrono::NaiveDate;

use Vec;
use crate::data::event::*;

pub struct Symbol{
    pub date: NaiveDate,
    pub symbol: String,

    pub events: Vec<Event>
}

impl Symbol{
    pub fn new(date: &NaiveDate, ticker: &str) -> Self{
        Symbol{
            date: date.clone(),
            symbol: ticker.to_string(),
            events: Vec::new()
        }
    }
}
