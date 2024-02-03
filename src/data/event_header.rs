use chrono::NaiveDateTime;

pub trait EventHeader{
    fn get_timestamp(&self) -> NaiveDateTime;
    fn get_type(&self) -> &str;
}


