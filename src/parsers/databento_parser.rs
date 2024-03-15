use core::panic;
use std::collections::HashMap;
use std::fs;


use chrono::{NaiveDate, NaiveDateTime};
use databento::dbn::decode::{DbnDecoder, DecodeRecordRef};
use databento::dbn::{RecordRef, RecordRefEnum};
use crate::writers::base_writer::BaseWriter;
use crate::paths::scratch;
use crate::data::event::*;
use crate::data::quote::Quote;
use crate::data::trade::Trade;
use crate::writers::parquet_writer::ParquetWriter;
use serde::Deserialize;
use log::info;
use heapless::sorted_linked_list::{SortedLinkedList, Min};

const BATCH_SIZE: usize = 1_000_000;

pub struct NasdaqParser<'a>{
    path_to_quotes: String,
    path_to_trades: String,
    path_to_symbology: String,
    writer: &'a mut Box<ParquetWriter>
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
    pub fn new(writer: &'a mut Box<ParquetWriter>, date: &NaiveDate) -> Self{
        NasdaqParser{
            path_to_quotes:  scratch::get_nasdaq_path_to_quotes(date),
            path_to_trades: scratch::get_nasdaq_path_to_quotes(date),
            path_to_symbology: scratch::get_symbology_path_to_trades(date),
            writer: writer
        }
    }

    fn load_symbology(&self, _date: &NaiveDate) -> HashMap<u32, String>{
        let file = fs::File::open(&self.path_to_symbology)
            .expect("Unable to open symbology file");
        let json: NasdaqSymbology = serde_json::from_reader(file)
            .expect("Uable to parse symbology file ");
        
        let mut result : HashMap<u32, String> = HashMap::new();

        for (key, value) in json.result{
            result.insert(value[0].s.parse().unwrap(), key);
        }
        
        result
    }

    #[allow(deprecated)]
    fn process_one(&self, record: &Option<RecordRef>, symbology: &HashMap<u32, String>) -> Option<Event>{
        match record {
            Some(r) =>
                match r.as_enum().unwrap(){
                    RecordRefEnum::Mbp1(mbp) => {
                        let timestamp = NaiveDateTime::from_timestamp_nanos(mbp.hd.ts_event as i64).unwrap();
                        Some(Event::Quote(Quote{
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
                        let timestamp = NaiveDateTime::from_timestamp_nanos(msg.hd.ts_event as i64).unwrap();
                        Some(Event::Trade(Trade{
                            symbol: symbology.get(&msg.hd.instrument_id).unwrap().to_string(),
                            exchange_date:  timestamp.to_string(),
                            price: msg.price as f64 * 0.000000001,
                            trade_timestamp: timestamp,
                            volume: msg.size as i64
                        }))
                    },
                    _ => panic!("no clue")
                },
            None => None
        }
    }

    

    pub(crate) fn parse_market_data(&mut self, date: &NaiveDate) {
        info!("Nasdaq market data parsing for date {}", date);
        let mut result: Vec<Event> = Vec::new();

        let symbology: HashMap<u32, String> = self.load_symbology(date);

        let mut trades_decoder = DbnDecoder::from_zstd_file(&self.path_to_trades).unwrap();
        let mut quotes_decorer = DbnDecoder::from_zstd_file(&self.path_to_quotes).unwrap();

        let latest_trade = self.process_one(&trades_decoder.decode_record_ref().unwrap(), &symbology).unwrap();
        let latest_quote = self.process_one(&quotes_decorer.decode_record_ref().unwrap(), &symbology).unwrap();

        let mut index = 0;
        let mut events_buffer: SortedLinkedList<Event, _, Min, 2> = SortedLinkedList::new_usize();
        events_buffer.push(latest_quote).unwrap();
        events_buffer.push(latest_trade).unwrap();
        while !events_buffer.is_empty() {   
            let min_event: Event = events_buffer.pop().unwrap();

            match min_event {
                Event::Quote(_) =>{
                    let next_event = self.process_one(&quotes_decorer.decode_record_ref().unwrap(), &symbology);
                    match next_event {
                        Some(e) => events_buffer.push(e).unwrap(),
                        None => ()
                    }
                },
                Event::Trade(_) =>{
                    let next_event = self.process_one(&trades_decoder.decode_record_ref().unwrap(), &symbology);
                    match next_event {
                        Some(e) => events_buffer.push(e).unwrap(),
                        None => ()
                    }
                }
            }
            result.push(min_event);
            if index != 0 && index % BATCH_SIZE == 0 {
                self.writer.write_matket_data(&result);
                result.clear();
                info!("Flushed after {} messages...", index);
            }

            index+=1;
        }
    }
}