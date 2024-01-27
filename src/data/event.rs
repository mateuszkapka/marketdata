use crate::data::trade;
use crate::data::quote;

pub enum Event{
    Trade(trade::Trade),
    Quote(quote::Quote)
}
