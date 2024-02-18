use core::panic;

use std::io::{BufRead, BufReader};


use crate::base_writer::BaseWriter;
use crate::data::event_header::*;
use crate::data::quote::*;

use crate::data::trade::*;
use crate::data::event::*;
use crate::parquet_writer::ParquetWriter;
use chrono::{Datelike, NaiveDate, NaiveDateTime, NaiveTime};
use std::fs::{self, DirEntry, File};
use std::path::Path;

pub struct WSEParser<'a> {
    path_to_data: &'a str,
    writer: &'a mut Box<ParquetWriter>
}

impl<'a> WSEParser<'a> {
    pub fn new(writer: &'a mut Box<ParquetWriter>) -> Self {
        WSEParser {
            path_to_data: "../WSE_Data/Sample_Warsaw_Kapka",
            writer: writer
        }
    }

    pub(crate) fn parse_market_data(&mut self, date: &NaiveDate) {
        let files_to_parse = self.get_files_to_parse(date);
        let mut result: Vec<Event> = Vec::new();

        for file in files_to_parse {
            self.process_symbol_file(file, &mut result, date);
        }

        result.sort_by(|a, b| NaiveDateTime::cmp(&a.get_timestamp(), &b.get_timestamp()));

        self.writer.write_matket_data(&result);
    }

    fn get_files_from_subpath(&self, directory: &str) -> Vec<DirEntry> {
        let mut result = Vec::new();
        let final_path = Path::new(self.path_to_data).join(directory);
        let final_path_str = final_path.clone();
        let final_dir = fs::read_dir(final_path)
            .expect(&format!("Unable to read folder {:?}", &final_path_str));
        for file in final_dir {
            result.push(file.unwrap())
        }

        result
    }

    fn get_files_to_parse(&self, date: &NaiveDate) -> Vec<DirEntry> {
        let mut quotes = self.get_files_from_subpath(
            format!(
                "{}_{:0>2}_{:0>2}_Q",
                date.year(),
                date.month0() + 1,
                date.day0() + 1
            )
            .as_str(),
        );
        let mut trades = self.get_files_from_subpath(
            format!(
                "{}_{:0>2}_{:0>2}_T",
                date.year(),
                date.month0() + 1,
                date.day0() + 1
            )
            .as_str(),
        );

        quotes.append(&mut trades);
        quotes
    }

    fn process_symbol_file(&self, file: DirEntry, events: &mut Vec<Event>, _date: &NaiveDate) {
        let event = get_type_from_filename(&file);
        let symbol = get_symbol_from_filename(&file);

        match event {
            'Q' => {
                let quotes = self.read_quotes(&file, &symbol);
                for quote in quotes {
                    events.push(Event::Quote(quote));
                }
            }
            'T' => {
                let trades = self.read_trades(&file, &symbol);
                for trade in trades {
                    events.push(Event::Trade(trade));
                }
            }
            _ => panic!("Invalid file type marker {}", event),
        }
    }

    fn read_quotes(&self, file: &DirEntry, symbol: &str) -> Vec<Quote> {
        let mut result = Vec::new();
        let file_handle = match File::open(file.path()) {
            Ok(fh) => fh,
            Err(err) => panic!("Unable to open file {:?}: {}", file.path(), err.to_string()),
        };

        let reader = BufReader::new(file_handle);
        for line_result in reader.lines() {
            let line = line_result.unwrap();
            let line_parts: Vec<&str> = line.split(",").into_iter().collect();
            match line_parts.len() {
                9 => {
                    let date = NaiveDate::parse_from_str(line_parts[0], "%m/%d/%Y").unwrap();
                    let time = NaiveTime::parse_from_str(
                        &line_parts[1][0..line_parts[1].len() - 4],
                        "%H:%M:%S",
                    )
                    .unwrap();
                    result.push(Quote {
                        symbol: symbol.to_string(),
                        quote_date: NaiveDateTime::new(date, time),
                        exchange_date: line_parts[2].to_string(),
                        bid_price: line_parts[4].parse().unwrap_or_default(),
                        bid_size: line_parts[5].parse().unwrap_or_default(),
                        ask_price: line_parts[6].parse().unwrap_or_default(),
                        ask_size: line_parts[7].parse().unwrap_or_default(),
                        market_period: line_parts[8].to_string(),
                    });
                }
                _ => panic!("Invalid line format! Cannot parse line {}", line),
            }
        }

        result
    }

    fn read_trades(&self, file: &DirEntry, symbol: &str) -> Vec<Trade> {
        let mut result = Vec::new();
        let file_handle = match File::open(file.path()) {
            Ok(fh) => fh,
            Err(err) => panic!("Unable to open file {:?}: {}", file.path(), err.to_string()),
        };

        let reader = BufReader::new(file_handle);
        for line_result in reader.lines() {
            let line = line_result.unwrap();
            let line_parts: Vec<&str> = line.split(",").into_iter().collect();
            match line_parts.len() {
                21 => {
                    let trade_date= NaiveDate::parse_from_str(line_parts[0], "%m/%d/%Y").unwrap();
                    let trade_time =  NaiveTime::parse_from_str(
                        &line_parts[1][0..line_parts[1].len() - 4],
                        "%H:%M:%S",
                    )
                    .unwrap();

                    result.push(Trade {
                        symbol: symbol.to_string(),
                        trade_timestamp: NaiveDateTime::new(trade_date, trade_time),
                        exchange_date: line_parts[2].to_string(),
                        price: line_parts[5].parse().unwrap_or_default(),
                        volume: line_parts[6].parse().unwrap_or_default(),
                    });
                }
                _ => panic!("Invalid line format! Cannot parse line {}", line),
            }
        }

        result
    }
}

fn get_symbol_from_filename(file: &DirEntry) -> String {
    let filename = file.file_name();
    let mut parts = filename.to_str().unwrap().split("_");
    let symbol = parts.nth(0).expect(
        format!(
            "Unable to parse event type from file '{:?}'. Not enough parts",
            &file.path()
        )
        .as_str(),
    );
    symbol.to_string()
}

fn get_type_from_filename(file: &DirEntry) -> char {
    let filename = file.file_name();
    let mut parts = filename.to_str().unwrap().split("_");
    let event_type = parts.nth(4).expect(
        format!(
            "Unable to parse event type from file '{:?}'. Not enough parts",
            &file.path()
        )
        .as_str(),
    );
    event_type.chars().nth(0).unwrap()
}
