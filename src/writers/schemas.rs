use std::collections::HashMap;

use arrow::datatypes::{DataType, Field, Schema};

#[allow(dead_code)]
pub fn get_symbology_schena() -> Schema{
    Schema::new(vec![
        Field::new("symbol", DataType::Utf8, true),
    ])
}

pub fn get_market_data_schema() -> Schema {
    Schema::new(vec![
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
    ])
}

#[allow(dead_code)]
pub fn get_aggregates_schema() -> Schema{
    Schema::new(vec![
        Field::new("symbol", DataType::Utf8, true),
        Field::new(
            "slice",
            DataType::Timestamp(arrow::datatypes::TimeUnit::Millisecond, None),
            false,
        ),
        Field::new("aggregate_name", DataType::Utf8, true),
        Field::new("value", DataType::Float64, true),
    ])
}

#[allow(dead_code)]
pub fn map_columns_to_indexes(schema: &Schema) -> HashMap<String, usize>{
    let mut result = HashMap::new();

    let mut index: usize = 0;
    for column in schema.all_fields(){
        result.insert(column.name().clone(), index);

        index += 1;
    }

    result
}