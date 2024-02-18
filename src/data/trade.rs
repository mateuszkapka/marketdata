use chrono::NaiveDateTime;

#[derive(Debug)]
pub struct Trade {
    pub symbol: String,
    pub trade_timestamp: NaiveDateTime,
    pub exchange_date: String,
    pub price: f64,
    pub volume: i64,
    // pub sales_condition: String,
    // pub market_mechanism: String,
    // pub trade_mode: String,
    // pub correction_indicator: String,
    // pub exclude_record_flag: String,
}
