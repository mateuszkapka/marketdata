use chrono::{NaiveDate, NaiveTime};

pub struct Trade {
    pub symbol: String,
    pub trade_date: NaiveDate,
    pub trade_time: NaiveTime,
    pub exchange_date: String,
    pub exchange_time: String,
    pub price: f64,
    pub volume: i64,
    // pub sales_condition: String,
    // pub market_mechanism: String,
    // pub trade_mode: String,
    // pub correction_indicator: String,
    // pub exclude_record_flag: String,
}
