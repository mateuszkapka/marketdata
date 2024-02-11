extern crate parquet;

use std::{
    collections::{HashMap, HashSet},
    fs::File,
    path::Path,
    sync::Arc,
};

use arrow::{
    array::*,
    datatypes::{DataType, Field, Schema},
};
use parquet::{
    arrow::ArrowWriter,
    basic::{Compression, ZstdLevel},
    file::properties::WriterProperties,
};

use crate::{
    data::{event::Event, event_header::EventHeader, symbol::Symbol},
    writers::base_writer::*,
};

pub struct ParquetWriter {}

impl BaseWriter for ParquetWriter {
    fn write_matket_data(&self, dataset: &Vec<Event>, result_filename: &str) {
        let props = WriterProperties::builder()
            .set_compression(Compression::ZSTD(
                <ZstdLevel as std::default::Default>::default(),
            ))
            .build();

        let schema = Schema::new(vec![
            Field::new(
                "timestamp",
                DataType::Timestamp(arrow::datatypes::TimeUnit::Millisecond, None),
                false,
            ),
            Field::new("symbol", DataType::Utf8, true),
            Field::new("bid_price", DataType::Float64, true),
            Field::new("bid_size", DataType::Int64, true),
            Field::new("ask_price", DataType::Float64, true),
            Field::new("ask_size", DataType::Int64, true),
            Field::new("market_period", DataType::Utf8, true),
            Field::new("trade_price", DataType::Float64, true),
            Field::new("trade_volume", DataType::Int64, true),
            Field::new("type", DataType::Utf8, true),
        ]);

        let mut writer = ArrowWriter::try_new(
            File::create(&Path::new(result_filename)).unwrap(),
            Arc::new(schema),
            Some(props),
        )
        .unwrap();

        println!("Writing data...");
        let mut index = 0;
        let chunk_size = 10000;
        let total_chunks = dataset.len() / chunk_size;
        let even_chunkgs = dataset.chunks(chunk_size);
        for single_chunk in even_chunkgs {
            index += 1;
            println!("Writing chunk {}/{}", index, total_chunks);
            let mut timestamp_data: Vec<i64> = Vec::new();
            let mut symbol_data: Vec<&str> = Vec::new();
            let mut bid_price_data: Vec<Option<f64>> = Vec::new();
            let mut bid_size_data: Vec<Option<i64>> = Vec::new();
            let mut ask_price_data: Vec<Option<f64>> = Vec::new();
            let mut ask_size_data: Vec<Option<i64>> = Vec::new();
            let mut market_period_data: Vec<Option<&str>> = Vec::new();
            let mut trade_price_data: Vec<Option<f64>> = Vec::new();
            let mut trade_volume_data: Vec<Option<i64>> = Vec::new();
            let mut type_data: Vec<&str> = Vec::new();
            for event in single_chunk {
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

            writer
                .write(&batch)
                .expect("Unable to write the next batch!");
        }

        writer.close().unwrap();
    }

    fn write_symbology(&self, symbols: &HashSet<String>, result_filename: &str) {
        let props = WriterProperties::builder()
            .set_compression(Compression::ZSTD(
                <ZstdLevel as std::default::Default>::default(),
            ))
            .build();

        let schema = Schema::new(vec![Field::new("symbol", DataType::Utf8, true)]);

        let mut writer = ArrowWriter::try_new(
            File::create(&Path::new(result_filename)).unwrap(),
            Arc::new(schema),
            Some(props),
        )
        .unwrap();

        let symbols = StringArray::from_iter_values(symbols);

        let batch =
            RecordBatch::try_from_iter(vec![("symbol", Arc::new(symbols) as ArrayRef)]).unwrap();

        writer
            .write(&batch)
            .expect("Unable to write the next batch!");
        writer.close().unwrap();
    }
}

