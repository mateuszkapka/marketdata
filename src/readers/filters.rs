
use std::{collections::HashMap, usize};

use parquet::record::{Row, RowAccessor};




pub trait ParquetFilter<'a>{
    fn should_filter_row(&self, row: &Row, column_mappng: &HashMap<String, usize> ) -> bool;
    fn get_symbol(&self) -> Option<&'a str>;
}

#[derive(Copy, Clone)]
pub struct NoOpFilter{

}

impl<'a> ParquetFilter<'a> for NoOpFilter{
    fn should_filter_row(&self, _: &Row, _: &HashMap<String, usize>) -> bool {
        false
    }

    fn get_symbol(&self) -> Option<&'a str>{
        None
    }
}

#[derive(Copy, Clone)]
pub struct SymbolFilter <'a>{
    symbol: &'a str
}

impl<'a> SymbolFilter<'a> {
    #[allow(dead_code)]
    pub fn new(symbol: &'a str) -> Self{
        SymbolFilter{
            symbol: symbol
        }
    }
}

impl<'a> ParquetFilter<'a> for SymbolFilter<'a>{
    fn should_filter_row(&self, row: &Row, column_mappng: &HashMap<String, usize>) -> bool {
        row.get_string(*column_mappng.get("symbol").unwrap()).unwrap() != &self.symbol
    }

    fn get_symbol(&self) -> Option<&'a str> {
        Some(self.symbol)
    }
}
