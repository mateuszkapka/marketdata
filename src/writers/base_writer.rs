use std::collections::HashSet;

use crate::data::event::Event;

pub trait BaseWriter {
    fn write_matket_data(&mut self, dataset: &Vec<Event>);
    fn write_symbology(&mut self, symbology: HashSet<String>);
    fn finalize(self);
}
