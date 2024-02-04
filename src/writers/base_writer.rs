use std::collections::HashMap;

use crate::data::symbol::Symbol;

pub trait BaseWriter{
    fn write_matket_data(&self, dataset: &HashMap<String, Symbol>, result_filename: &str);
    fn write_symbology(&self, symbols: &Vec<String>, result_filename: &str);
}