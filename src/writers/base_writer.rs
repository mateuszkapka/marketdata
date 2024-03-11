use std::collections::HashSet;

use crate::{aggregates::aggregate_framework::AggregateValue, data::event::Event};

pub trait BaseWriter {
    fn write_matket_data(&mut self, dataset: &Vec<Event>);
    fn write_symbology(&mut self, symbology: HashSet<String>);
    fn write_aggregates(&mut self, agregates: &Vec<AggregateValue>);
    fn finalize(self);
}
