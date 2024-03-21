use crate::data::{quote::Quote, trade::Trade};

use super::aggregate_framework::AggregateFrameworkContext;

pub trait Aggregate{
    fn on_quote(&mut self, quote: &Quote);
    fn on_trade(&mut self, quote: &Trade);
    fn compute_slice(&self,context: &AggregateFrameworkContext) -> f64;

    fn get_name(&self) -> &str;
    fn get_symbol(&self) -> &str;
}

pub trait AggregateNew{
    fn new(symbol: &str) -> Self;
}

pub const DEFAULT_AGGREGATES: &'static [&'static str] = &["Volume"];


