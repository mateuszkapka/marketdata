

use chrono::{Duration, NaiveDate};
use chrono::{NaiveDateTime, NaiveTime};


pub trait SliceSchedule{
    fn trigger_maybe(&mut self, current_time: &NaiveDateTime) -> Option<&NaiveDateTime>;
}


pub struct WallClockSliceSchedule{
    last_slice_index: usize,
    schedule: Vec<NaiveDateTime>
}
impl WallClockSliceSchedule{
    fn generate_schedule(date: &NaiveDate) -> Vec<NaiveDateTime>{
        let mut result = Vec::new();
        
        let mut time = NaiveDateTime::new(date.clone(), NaiveTime::from_hms_milli_opt(9, 0, 0, 0).unwrap());
        let end_time =  NaiveDateTime::new(date.clone(), NaiveTime::from_hms_milli_opt(15, 50, 0, 0).unwrap());
        
        result.push(time.clone());
        loop{
            time += Duration::minutes(5);
            if time > end_time{
                break;
            }
            result.push(time.clone());
            
        }

        result
    }

    pub fn new(date: &NaiveDate) -> Self {
        WallClockSliceSchedule{
            last_slice_index:0 ,
            schedule: Self::generate_schedule(date)
        }
    }
}

impl SliceSchedule for WallClockSliceSchedule{
    fn trigger_maybe(&mut self, current_time: &NaiveDateTime) -> Option<&NaiveDateTime> {
        if self.last_slice_index >= self.schedule.len(){
            return None;
        }

        if current_time >= &self.schedule[self.last_slice_index]{
            self.last_slice_index += 1;
            return Some(&self.schedule[self.last_slice_index-1]);
        }

        None
    }
}