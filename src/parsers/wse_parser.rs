use std::collections::HashMap;

use chrono::NaiveDate;
use crate::data::symbol::*;

pub struct WSEParser{

}

impl WSEParser{
    pub fn new() -> Self{
        WSEParser{}
    }

    pub(crate) fn parse_market_data(&self, date: &NaiveDate) -> HashMap<String,Symbol> {
       !todo!()
    }
}