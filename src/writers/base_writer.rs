use std::collections::HashMap;

use crate::data::symbol::Symbol;

pub trait BaseWriter{
    fn write_symbol(&self, dataset: &HashMap<String, Symbol>);
}