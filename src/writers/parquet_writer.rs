extern crate parquet;

use std::{
    collections::HashSet, fs::File, path::Path, sync::Arc
};

use arrow::{
    array::*, datatypes::Schema,
};
use log::info;
use parquet::{
    arrow::ArrowWriter,
    basic::{Compression, ZstdLevel},
    file::properties::WriterProperties,
};

use crate::{
    common::get_market_data_schema, data::{event::Event, event_header::EventHeader}, writers::base_writer::*
};

pub struct ParquetWriter {
    writer: ArrowWriter<File>
}

impl ParquetWriter{
    pub fn new(result_filename: String, schema: Schema) -> Self{
        let props = WriterProperties::builder()
        .set_compression(Compression::ZSTD(
            <ZstdLevel as std::default::Default>::default(),
        ))
        .build();


        let writer = ArrowWriter::try_new(
            File::create(&Path::new(&result_filename)).unwrap(),
            Arc::new(schema),
            Some(props),
        )
        .unwrap();

        ParquetWriter{
            writer
        }
    }
}

impl BaseWriter for ParquetWriter {
    fn write_matket_data(&mut self, dataset: &Vec<Event>) {
        info!("Starting to write data");
        let mut timestamp_data: Vec<i64> = Vec::new();
        timestamp_data.reserve(dataset.len());
        let mut symbol_data: Vec<&str> = Vec::new();
        symbol_data.reserve(dataset.len());
        let mut bid_price_data: Vec<Option<f64>> = Vec::new();
        bid_price_data.reserve(dataset.len());
        let mut bid_size_data: Vec<Option<i64>> = Vec::new();
        bid_size_data.reserve(dataset.len());
        let mut ask_price_data: Vec<Option<f64>> = Vec::new();
        ask_price_data.reserve(dataset.len());
        let mut ask_size_data: Vec<Option<i64>> = Vec::new();
        ask_size_data.reserve(dataset.len());
        let mut market_period_data: Vec<Option<&str>> = Vec::new();
        market_period_data.reserve(dataset.len());
        let mut trade_price_data: Vec<Option<f64>> = Vec::new();
        trade_price_data.reserve(dataset.len());
        let mut trade_volume_data: Vec<Option<i64>> = Vec::new();
        trade_volume_data.reserve(dataset.len());
        let mut type_data: Vec<&str> = Vec::new();
        type_data.reserve(dataset.len());

        for event in dataset {
            timestamp_data.push(event.get_timestamp().timestamp_millis());
            symbol_data.push(event.get_symbol());
            type_data.push(event.get_type());
            match event {
                Event::Quote(quote) => {
                    bid_price_data.push(Some(quote.bid_price));
                    bid_size_data.push(Some(quote.bid_size));
                    ask_price_data.push(Some(quote.ask_price));
                    ask_size_data.push(Some(quote.ask_size));
                    market_period_data.push(Some(&quote.market_period));
                    trade_price_data.push(None);
                    trade_volume_data.push(None);
                }
                Event::Trade(trade) => {
                    bid_price_data.push(None);
                    bid_size_data.push(None);
                    ask_price_data.push(None);
                    ask_size_data.push(None);
                    market_period_data.push(None);
                    trade_price_data.push(Some(trade.price));
                    trade_volume_data.push(Some(trade.volume));
                }
            }
        }

        let timestamps = TimestampMillisecondArray::from(timestamp_data);
        let symbols = StringArray::from(symbol_data);
        let bid_prices = Float64Array::from(bid_price_data);
        let bid_sizes = Int64Array::from(bid_size_data);
        let ask_prices = Float64Array::from(ask_price_data);
        let ask_sizes = Int64Array::from(ask_size_data);
        let market_periods = StringArray::from(market_period_data);
        let trade_prices = Float64Array::from(trade_price_data);
        let trade_volumes = Int64Array::from(trade_volume_data);
        let types = StringArray::from(type_data);

        let batch = RecordBatch::try_from_iter(vec![
            ("timestamp", Arc::new(timestamps) as ArrayRef),
            ("symbol", Arc::new(symbols) as ArrayRef),
            ("bid_price", Arc::new(bid_prices) as ArrayRef),
            ("bid_size", Arc::new(bid_sizes) as ArrayRef),
            ("ask_price", Arc::new(ask_prices) as ArrayRef),
            ("ask_size", Arc::new(ask_sizes) as ArrayRef),
            ("market_period", Arc::new(market_periods) as ArrayRef),
            ("trade_price", Arc::new(trade_prices) as ArrayRef),
            ("trade_volume", Arc::new(trade_volumes) as ArrayRef),
            ("type", Arc::new(types) as ArrayRef),
        ])
        .unwrap();
        
        info!("Data prepared, writing to disk");
        self.writer
            .write(&batch)
            .expect("Unable to write the next batch!");

        info!("Writing finished")
    }


    fn finalize(self) {
        self.writer.close().unwrap();
    }

    fn write_symbology(&mut self, symbology: HashSet<String>) {
        info!("Starting to write data");

        let symbols = StringArray::from(symbology.iter().map(|value| value.clone()).collect::<Vec<String>>());

        let batch = RecordBatch::try_from_iter(vec![
            ("symbol", Arc::new(symbols) as ArrayRef),
        ])
        .unwrap();
        
        info!("Data prepared, writing to disk");
        self.writer
            .write(&batch)
            .expect("Unable to write the next batch!");

        info!("Writing finished")
    }
}
