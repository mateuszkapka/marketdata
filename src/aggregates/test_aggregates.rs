use crate::data::{quote::Quote, trade::Trade};

use super::{aggregate_base::{Aggregate, AggregateNew}, aggregate_framework::{AggregateFrameworkContext, AggregateReference}};




pub struct SimpleAggregate{
    symbol: String,
    volume_ref: AggregateReference
}

impl Aggregate for SimpleAggregate{
     fn on_quote(&mut self, _quote: &Quote) {
        // println!("quote for {}|{}", quote.quote_date, quote.symbol);
     } 
     
     fn on_trade(&mut self, _trade:  &Trade) {
        // println!("trade for {}|{}", trade.trade_timestamp, trade.symbol);
     }

     fn compute_slice(&self, _slice: &chrono::prelude::NaiveDateTime) -> f64 {
        self.volume_ref.this_slice()
     }

     fn get_name(&self) -> &str {
         "SimpleAggregate"
     }

     fn get_symbol(&self) -> &str {
         &self.symbol
     }
}

impl AggregateNew for SimpleAggregate{
    fn new(symbol: &str, context: &AggregateFrameworkContext) -> Self {
        SimpleAggregate{
            symbol: symbol.to_string(),
            volume_ref: context.get_aggregate_reference("Volume",symbol)
        }
    }
}


pub struct VolumeAggregate{
    symbol: String,
    total_volume: f64
}

impl Aggregate for VolumeAggregate{

     fn on_quote(&mut self, _quote: &Quote) {
        
     }
     
     fn on_trade(&mut self, trade: &Trade) {
        self.total_volume += trade.volume as f64;
     }

     fn compute_slice(&self, _slice: &chrono::prelude::NaiveDateTime) -> f64 {

        self.total_volume
     }

     fn get_name(&self) -> &str {
         "VolumeAggregate"
     }

     fn get_symbol(&self) -> &str {
         &self.symbol
     }
}

impl AggregateNew for VolumeAggregate{
    fn new(symbol: &str,  _: & AggregateFrameworkContext) -> Self {
        VolumeAggregate{
            symbol: symbol.to_string(),
            total_volume: 0.0
        }
    }
}