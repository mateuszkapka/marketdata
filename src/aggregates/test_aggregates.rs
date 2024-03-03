use crate::data::{quote::Quote, trade::Trade};

use super::aggregate_base::Aggregate;



pub struct SimpleAggregate{

}

impl Aggregate for SimpleAggregate{
     fn on_quote(&self, quote: &Quote) {
         println!("quote for {}|{}", quote.quote_date, quote.symbol);
     }
     
     fn on_trade(&self, trade: &Trade) {
         println!("trade for {}|{}", trade.trade_timestamp, trade.symbol);
     }
}

impl Default for SimpleAggregate{
    fn default() -> Self {
        SimpleAggregate{}
    }
}