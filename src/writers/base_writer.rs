use std::collections::HashSet;

use crate::data::event::Event;

pub trait BaseWriter {
    fn write_matket_data(&self, dataset: &Vec<Event>, result_filename: &str);
    fn write_symbology(&self, symbols: &HashSet<String>, result_filename: &str);
}
