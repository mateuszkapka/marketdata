use chrono::{NaiveTime, NaiveDate};

pub struct Quote{
    pub quote_date: NaiveDate,
    pub quote_time: NaiveTime,
    pub exchange_date: String,
    pub exchange_time: String,
    pub bid_price: f64,
    pub bid_size: i64,
    pub ask_price: f64,
    pub ask_size: i64,
    pub market_period: String
}