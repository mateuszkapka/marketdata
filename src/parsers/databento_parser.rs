use core::panic;
use std::collections::HashMap;
use std::fs;

use chrono::{NaiveDate, NaiveDateTime};
use databento::dbn::RecordRefEnum;
use databento::historical::timeseries::AsyncDbnDecoder;
use crate::data::event::*;
use crate::data::quote::Quote;
use futures::executor::block_on;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct NasdaqParser<'a>{
    path_to_data: &'a str,
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
            path_to_data: "Sample_Warsaw_Kapka/xnas-itch-20240122.mbp-1.dbn.zst",
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

    fn parse_single_file(&self, _date: &NaiveDate, result: & mut Vec<Event>, symbology: &HashMap<u32, String>){
        let mut decoder = block_on(AsyncDbnDecoder::from_zstd_file(self.path_to_data)).unwrap();
       
        
         while let Some(r) = block_on(decoder.decode_record_ref()).unwrap() {
            
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
                _ => panic!("no clue")
            }
        }
    }

    pub(crate) fn parse_market_data(&self, date: &NaiveDate) -> Vec<Event> {
        let mut result: Vec<Event> = Vec::new();

        let symbology = self.load_symbology(date);
        self.parse_single_file(date, &mut result, &symbology);

        result
    }
}