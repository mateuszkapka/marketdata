use chrono::NaiveDateTime;
use simple_error::SimpleError;

use crate::data::{quote::Quote, trade::Trade};

use super::test_aggregates::{SimpleAggregate, VolumeAggregate};

pub trait Aggregate{
    fn on_quote(&mut self, quote: &Quote);
    fn on_trade(&mut self, quote: &Trade);
    fn compute_slice(&self, slice: &NaiveDateTime) -> f64;

    fn get_name(&self) -> &str;
    fn get_symbol(&self) -> &str;
}

pub trait AggregateNew{
    fn new(symbol: &str) -> Self;
}

pub const DEFAULT_AGGREGATES: &'static [&'static str] = &["Volume"];

pub fn aggregate_from_name(agg_name: &str, symbol: &str) -> Result<Box<dyn Aggregate>, SimpleError>  {
    match agg_name {
        "Volume" => Ok(Box::new(VolumeAggregate::new(symbol))),
        "Test" => Ok(Box::new(SimpleAggregate::new(symbol))),
        _ => Err(SimpleError::new(format!("Unknown aggregate type {}", agg_name)))
    }
}



