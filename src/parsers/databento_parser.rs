use core::panic;
use std::collections::HashMap;
use std::fs;

use chrono::{NaiveDate, NaiveDateTime};
use databento::dbn::decode::{DbnDecoder, DecodeRecordRef};
use databento::dbn::RecordRefEnum;
use crate::data::event::*;
use crate::data::event_header::EventHeader;
use crate::data::quote::Quote;
use crate::data::trade::Trade;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct NasdaqParser<'a>{
    path_to_quotes: &'a str,
    path_to_trades: &'a str,
    path_to_symbology: &'a str
}

#[derive(Deserialize)]
pub struct NasdaqSymbologyRow{
    pub d0: String,
    pub d1: String,
    pub s: String
}

#[derive(Deserialize)]
pub struct NasdaqSymbology{
    pub result: HashMap<String, Vec<NasdaqSymbologyRow>>
}


impl<'a> NasdaqParser<'a>{
    pub fn new() -> Self{
        NasdaqParser{
            path_to_quotes: "sample_nasdaq_databento/mbp/xnas-itch-20240122.mbp-1.dbn.zst",
            path_to_trades: "sample_nasdaq_databento/tbbo/xnas-itch-20240122.tbbo.dbn.zst",
            path_to_symbology: "sample_nasdaq_databento/tbbo/symbology (1).json"
        }
    }

    fn load_symbology(&self, _date: &NaiveDate) -> HashMap<u32, String>{
        let file = fs::File::open(self.path_to_symbology)
            .expect("Unable to open symbology file");
        let json: NasdaqSymbology = serde_json::from_reader(file)
            .expect("Uable to parse symbology file ");
        
        let mut result : HashMap<u32, String> = HashMap::new();

        for (key, value) in json.result{
            result.insert(value[0].s.parse().unwrap(), key);
        }
        
        result
    }

    fn parse_single_file(&self, _date: &NaiveDate, symbology: &HashMap<u32, String>, path_to_data: &str, result: &mut Vec<Event>) {
        let mut decoder = DbnDecoder::from_zstd_file(path_to_data).unwrap();
         while let Some(r) = decoder.decode_record_ref().unwrap() {
            match r.as_enum().unwrap(){
                RecordRefEnum::Mbp1(mbp) => {
                    let timestamp = NaiveDateTime::from_timestamp_micros(mbp.hd.ts_event as i64).unwrap();
                    result.push(Event::Quote(Quote{
                        symbol: symbology.get(&mbp.hd.instrument_id).unwrap().to_string(),
                        quote_date: timestamp,
                        ask_price: mbp.levels[0].ask_px as f64 * 0.000000001,
                        ask_size: mbp.levels[0].ask_sz as i64,
                        bid_price: mbp.levels[0].bid_px as f64 * 0.000000001,
                        bid_size: mbp.levels[0].bid_sz as i64,
                        exchange_date: timestamp.to_string(),
                        market_period: "".to_string()
                    }))
                },
                RecordRefEnum::Trade(msg) => {
                    let timestamp = NaiveDateTime::from_timestamp_micros(msg.hd.ts_event as i64).unwrap();
                    result.push(Event::Trade(Trade{
                        symbol: symbology.get(&msg.hd.instrument_id).unwrap().to_string(),
                        exchange_date:  timestamp.to_string(),
                        price: msg.price as f64 * 0.000000001,
                        trade_timestamp: timestamp,
                        volume: msg.size as i64
                    }))
                },
                _ => panic!("no clue")
            }
        }
    }

    pub(crate) fn parse_market_data(&self, date: &NaiveDate) -> Vec<Event> {
        let mut result: Vec<Event> = Vec::new();

        let symbology: HashMap<u32, String> = self.load_symbology(date);

        println!("Reading trades..");
        self.parse_single_file(&date, &symbology, self.path_to_trades, &mut result);
        println!("Reading quotes..");
        self.parse_single_file(&date, &symbology, self.path_to_quotes, &mut result);
        
        result.sort_by(|a, b| NaiveDateTime::cmp(&a.get_timestamp(), &b.get_timestamp()));
        result
    }
}