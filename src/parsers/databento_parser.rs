use core::panic;
use std::collections::HashMap;
use std::io::{BufRead, BufReader};

use chrono::{Datelike, NaiveDate, NaiveDateTime, NaiveTime};
use crate::data::event::*;
use crate::data::symbol::*;
use std::fs::{self, DirEntry, File};
use std::path::Path;
use crate::data::trade::*;
use crate::data::quote::*;
use crate::data::event_header::*;

use databento::{
    dbn::{
        decode::AsyncDbnDecoder,
        encode::dbn::AsyncEncoder as AsyncDbnEncoder, Schema,
        TradeMsg,
    },
    historical::timeseries::GetRangeParams,
    HistoricalClient,
};
use time::macros::datetime;
use tokio::fs::File;


pub struct nasdaqParser<'a>{
    path_to_data: &'a str
}


impl<'a> nasdaqParser<'a>{
    pub fn new() -> Self{
        nasdaqParser{
            path_to_data: "Sample_Warsaw_Kapka/xnas-itch-20240122.mbp-1.dbn.zst"
            //
        }
    }

    pub(crate) fn parse_market_data(&self, date: &NaiveDate) -> Vec<Event> {
        let mut decoder = AsyncDbnDecoder::from_file(path).await?;
        for r in decoder.decode_record_ref().await? {
           
        }
    }
}