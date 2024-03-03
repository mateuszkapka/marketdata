use crate::data::{quote::Quote, trade::Trade};

pub trait Aggregate{
    fn on_quote(&self, quote: &Quote);
    fn on_trade(&self, quote: &Trade);
}