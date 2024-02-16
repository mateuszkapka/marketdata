use chrono::NaiveDateTime;

pub struct Quote {
    pub symbol: String,
    pub quote_date: NaiveDateTime,
    pub exchange_date: String,
    pub bid_price: f64,
    pub bid_size: i64,
    pub ask_price: f64,
    pub ask_size: i64,
    pub market_period: String,
}
