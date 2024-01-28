use chrono::NaiveDateTime;

pub trait EventHeader{
    fn get_timestamp(&self) -> NaiveDateTime;
}


