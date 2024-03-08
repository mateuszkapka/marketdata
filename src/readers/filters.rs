
use std::{collections::HashMap, usize};

use parquet::record::{Row, RowAccessor};

use crate::data::{event::Event, event_header::EventHeader};




pub trait ParquetFilter{
    fn should_filter_row(&self, row: &Row, column_mappng: &HashMap<String, usize> ) -> bool;
}

#[derive(Copy, Clone)]
pub struct NoOpFilter{

}

impl ParquetFilter for NoOpFilter{
    fn should_filter_row(&self, _: &Row, _: &HashMap<String, usize>) -> bool {
        false
    }
}

#[derive(Copy, Clone)]
pub struct SymbolFilter <'a>{
    symbol: &'a str
}

impl<'a> SymbolFilter<'a> {
    pub fn new(symbol: &'a str) -> Self{
        SymbolFilter{
            symbol: symbol
        }
    }
}

impl<'a> ParquetFilter for SymbolFilter<'a>{
    fn should_filter_row(&self, row: &Row, column_mappng: &HashMap<String, usize>) -> bool {
        row.get_string(*column_mappng.get("symbol").unwrap()).unwrap() != &self.symbol
    }
}