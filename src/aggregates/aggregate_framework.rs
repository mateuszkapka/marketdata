use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use chrono::{Days, NaiveDate, NaiveDateTime, NaiveTime};
use simple_error::SimpleError;

use super::aggregate_base::{Aggregate, AggregateNew, DEFAULT_AGGREGATES};
use super::test_aggregates::{SimpleAggregate, VolumeAggregate};
use crate::paths::scratch::{get_aggregates_path, get_normalised_path, get_symbology_path};
use crate::data::event::Event;
use crate::data::event_header::EventHeader;
use crate::parsers::parser::ParserType;
use crate::readers::filters::{ParquetFilter, SymbolFilter};
use crate::readers::parquet_reader::{ParquetReader, ParquetStreamReader};
use crate::aggregates::schedule::SliceSchedule;
use crate::aggregates::schedule::WallClockSliceSchedule;

#[derive(Debug, Clone)]
pub struct AggregateValue{
    pub symbol: String,
    pub slice: NaiveDateTime,
    pub aggregate_name: String,
    pub value: f64
}

pub struct AggregateFramework<'a> {
    aggregates: HashMap<String, Vec<Box<dyn Aggregate + 'a>>>,
    symbology: Vec<String>,
    date: NaiveDate,
    parser_type: &'a ParserType,
    slice_schedule: Box<dyn SliceSchedule>,
    read_cache: Rc<RefCell<AggregateReadCache>>,
    filter: Option<SymbolFilter<'a>>,
}

impl<'a> AggregateFramework<'a> {

    pub fn new(parser_type: &'a ParserType, date: &NaiveDate, filter: Option<SymbolFilter<'a>>) -> Self{ 
        let framework = AggregateFramework{
            aggregates: HashMap::new(),
            symbology: Self::read_symbology(parser_type, date),
            date: date.clone(),
            read_cache: Rc::new(RefCell::new(AggregateReadCache::new(date, parser_type))),
            parser_type,
            slice_schedule: Box::new(WallClockSliceSchedule::new(date)),
            filter
        };
        framework
    }

    fn read_symbology(parser_type: &ParserType, date: &NaiveDate) -> Vec<String> {
        let reader = ParquetReader{};

        let filepath = get_symbology_path(date, &parser_type);
        reader.read_symbology(&filepath)
    }

    pub fn register_aggregate<TAggregate> (&mut self)
    where TAggregate: Aggregate + AggregateNew + 'a{
        for symbol in &self.symbology {
            let aggregates_vector: & mut Vec<Box<dyn Aggregate>> = match self.aggregates.get_mut(&symbol[..]) {
                None => {
                    self.aggregates.insert(symbol.clone(), Vec::new());
                    self.aggregates.get_mut(&symbol.clone()).unwrap()
                },
                Some(value) => value
            };

            aggregates_vector.push(Box::new(TAggregate::new(&symbol)));
        }
    }

    pub fn register_aggregate_list_by_name(&mut self, agg_names: Vec<&str>) -> Result<(), SimpleError>{
        for agg in agg_names{
            for symbol in &self.symbology {
                let aggregates_vector: & mut Vec<Box<dyn Aggregate>> = match self.aggregates.get_mut(&symbol[..]) {
                    None => {
                        self.aggregates.insert(symbol.clone(), Vec::new());
                        self.aggregates.get_mut(symbol).unwrap()
                    },
                    Some(value) => value
                };
    
                
    
                aggregates_vector.push(match agg {
                    "Volume" => Box::new(VolumeAggregate::new(symbol)) as Box<dyn Aggregate>,
                    "Test" => Box::new(SimpleAggregate::new(symbol)),
                    _ => return Err(SimpleError::new(format!("Unknown aggregate type {}", agg)))
                });
            }
        }

        Ok(())
    }
    
    pub fn register_aggregate_by_name(&mut self, agg_name: &str) -> Result<(), SimpleError>{
        self.register_aggregate_list_by_name(vec![agg_name])?;

        Ok(())
    }

    pub fn run(&mut self)  -> Result<Vec<AggregateValue>, SimpleError>{

        let mut reader = ParquetStreamReader{
            filter: self.filter.clone(),
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
                                let cache = self.read_cache.clone();
                                let context = AggregateFrameworkContext::new(slice_time, cache);
                                let value = agg.as_mut().compute_slice(&context);
                                drop(context);
                                {
                                    self.read_cache.borrow_mut().push(AggregateValue{
                                        symbol: agg.get_symbol().to_string(),
                                        aggregate_name: agg.get_name().to_string(),
                                        slice: slice_time.clone(),
                                        value
                                    });
                                }
                            }
                        },
                        None => ()
                    }
                }
            }
        };

        let filter_symbol = match self.filter {
            Some(f) => f.get_symbol(),
            None => None
        };
        
        let filename = get_normalised_path(&self.date, &self.parser_type, filter_symbol);
        reader.read_market_data(&filename)?;
        Ok(self.read_cache.try_borrow().unwrap().aggregates.to_vec())

    }

    pub fn register_default_aggregates(&mut self) -> Result<(),SimpleError>{
        self.register_aggregate_list_by_name(DEFAULT_AGGREGATES.to_vec())?;
    
        Ok(())
    }

}


pub struct AggregateReadCache{
    current_date: NaiveDate,
    aggregates: Vec<AggregateValue>,
    historical_aggregate_cache: HashMap<NaiveDate, Vec<AggregateValue>>,
    parser_type: ParserType
}

impl AggregateReadCache{
    pub fn read_aggregate(_agg_name: &str, _slice: &NaiveTime, _date: &NaiveDate) -> f64{
        unimplemented!();
    }

    pub fn push(&mut self, value: AggregateValue){
        self.aggregates.push(value);
    }

    pub fn get_todays_aggregates(&self) -> &Vec<AggregateValue>{
        &self.aggregates
    }
    
    pub fn get_aggregates_for_date(&self, date: &NaiveDate) -> &Vec<AggregateValue>{
        &self.historical_aggregate_cache.get(date).unwrap()
    }

    pub fn new(date: &NaiveDate, parser_type: &ParserType) -> Self{
        AggregateReadCache{
            aggregates: Vec::new(),
            current_date: date.clone(),
            historical_aggregate_cache: HashMap::new(),
            parser_type: parser_type.clone()
        }
    }

    pub fn ensure_date(&mut self, date: &NaiveDate){
        let reader = ParquetReader{};

        let filepath = get_aggregates_path(date, &self.parser_type, None);
        let aggs = reader.read_aggregates(&filepath);
        self.historical_aggregate_cache.insert(date.clone(), aggs);
    }

}

pub struct AggregateFrameworkContext<'a>{
    cache: Rc<RefCell<AggregateReadCache>>,
    slice: &'a NaiveDateTime
}

impl<'a> AggregateFrameworkContext<'a>{
    fn new(slice: &'a NaiveDateTime, cache: Rc<RefCell<AggregateReadCache>>) -> Self{
        AggregateFrameworkContext{
            cache: cache,
            slice: slice
        }
    }

    pub fn agg_ref(&'a self, aggregate_name: &str, aggregate: &dyn Aggregate) -> AggregateReference {
        AggregateReference{
            aggregate_name: format!("{}Aggregate", aggregate_name), 
            slice: self.slice,
            context: self,
            symbol: aggregate.get_symbol().to_string(),
        }
    }


    pub fn ensure_date(&mut self, date: &NaiveDate){
        self.cache.borrow_mut().ensure_date(date);
    }
}

pub struct AggregateReference<'a>{
    aggregate_name: String,
    symbol: String,
    slice: &'a NaiveDateTime,
    context: &'a AggregateFrameworkContext<'a>
}

impl<'a> AggregateReference<'a>{
    pub fn this_slice(&self) -> f64{
        for value in self.context.cache.borrow_mut().get_todays_aggregates().iter().rev(){
            if value.aggregate_name == self.aggregate_name && value.symbol == self.symbol{
                return value.value;
            }
        }

        panic!("Unable to find this slice for aggregate {} and symbol {} ", self.aggregate_name, self.symbol)
    } 

    pub fn prev_eod(&mut self) -> f64{
        let date = &self.slice.checked_sub_days(Days::new(1)).unwrap().date();
        self.context.cache.borrow_mut().ensure_date(date);

        for value in self.context.cache.borrow_mut().get_aggregates_for_date(date).iter().rev(){
            if value.aggregate_name == self.aggregate_name && value.symbol == self.symbol{
                return value.value;
            }
        }

        0.0
    }
}