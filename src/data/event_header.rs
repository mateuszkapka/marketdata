use chrono::{NaiveDateTime, NaiveTime, NaiveDate};

pub trait EventHeader{
    fn getTimestamp(&self) -> NaiveDateTime;
}


