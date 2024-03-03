use std::collections::HashMap;

use chrono::{NaiveDate, NaiveDateTime};

use super::aggregate_base::{Aggregate, AggregateNew};
use crate::common::{get_normalised_path, get_symbology_path};
use crate::data::event::Event;
use crate::data::event_header::EventHeader;
use crate::parsers::parser::ParserType;
use crate::readers::parquet_reader::{ParquetReader, ParquetStreamReader};
use crate::aggregates::schedule::SliceSchedule;
use crate::aggregates::schedule::WallClockSliceSchedule;

pub struct AggregateValue{
    symbol: String,
    slice: NaiveDateTime,
    aggregate_name: String,
    value: f64
}

pub struct AggregateFramework<'a>{
    aggregates: HashMap<String, Vec<Box<dyn Aggregate + 'a>>>,
    symbology: Vec<String>,
    date: NaiveDate,
    parser_type: &'a ParserType,
    slice_schedule: Box<dyn SliceSchedule>,
    aggregate_values: Vec<AggregateValue>
}

impl<'a> AggregateFramework<'a>{
    pub fn new(parser_type: &'a ParserType, date: &NaiveDate) -> Self{
        
        AggregateFramework{
            aggregates: HashMap::new(),
            symbology: Self::read_symbology(parser_type, date),
            date: date.clone(),
            parser_type,
            slice_schedule: Box::new(WallClockSliceSchedule::new(date)),
            aggregate_values: Vec::new()
        }
    }

    fn read_symbology(parser_type: &ParserType, date: &NaiveDate) -> Vec<String> {
        let reader = ParquetReader{};

        let filepath = get_symbology_path(date, &parser_type);
        reader.read_symbology(&filepath)
    }

    pub fn register_aggregate<TAggregate> (& mut self)
    where TAggregate: Aggregate + AggregateNew + 'a{
        for symbol in &self.symbology {
            let aggregates_vector: & mut Vec<Box<dyn Aggregate>> = match self.aggregates.get_mut(&symbol[..]) {
                None => {
                    self.aggregates.insert(symbol.clone(), Vec::new());
                    self.aggregates.get_mut(symbol).unwrap()
                },
                Some(value) => value
            };

            aggregates_vector.push(Box::new(TAggregate::new(&symbol)));
        }
    }

    pub fn run(&mut self)  -> &Vec<AggregateValue>{

        let mut reader = ParquetStreamReader{
            on_event: |event: Event| {
                match &event {
                    Event::Quote(quote) => {
                        match self.aggregates.get_mut(event.get_symbol()) {
                            Some(aggregates_to_run) => {
                                for agg in aggregates_to_run {
                                    agg.as_mut().on_quote(&quote);
                                }
                            },
                            None => ()
                        }
                    },
                    Event::Trade(trade) => {
                        match self.aggregates.get_mut(event.get_symbol()) {
                            Some(aggregates_to_run) => {
                                for agg in aggregates_to_run {
                                    agg.as_mut().on_trade(&trade);
                                }
                            },
                            None => ()
                        }
                    },
                }

                if let Some(slice_time) = self.slice_schedule.trigger_maybe(&event.get_timestamp()){
                    match self.aggregates.get_mut(event.get_symbol()) {
                        Some(aggregates_to_run) => {
                            for agg in aggregates_to_run {
                                let value = agg.as_mut().compute_slice(slice_time);
                                self.aggregate_values.push(AggregateValue{
                                    symbol: agg.get_symbol().to_string(),
                                    aggregate_name: agg.get_name().to_string(),
                                    slice: slice_time.clone(),
                                    value
                                });
                            }
                        },
                        None => ()
                    }
                }
            }
        };
        
        let filename = get_normalised_path(&self.date, &self.parser_type);
        reader.read_market_data(&filename);
        &self.aggregate_values

    }
}