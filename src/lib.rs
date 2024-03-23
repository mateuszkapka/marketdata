use std::str::FromStr;

use aggregates::aggregate_framework::AggregateFramework;
use chrono::NaiveDate;
use parsers::parser::ParserType;
use pyo3::prelude::*;

mod data;
mod parsers;
mod writers;
mod readers;
mod aggregates;
mod paths;
mod utils;

use pyo3_polars::PyDataFrame;
use polars::prelude::*;
use polars::df;
use readers::filters::SymbolFilter;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a * b).to_string())
}

#[pyfunction]
fn compute_aggregates(market: &str, symbol: Option<&str>, aggregates: Option<&str>) -> PyResult<PyDataFrame> {
    let source = ParserType::from_str(&market).expect("Invalid value for argument source!");
    let date = NaiveDate::from_ymd_opt(2024, 01, 23).unwrap();
    let filter = symbol.map_or_else(|| None, |x| Some(SymbolFilter::new(x)));
    let mut framework = AggregateFramework::new(&source, &date, filter);
    
    match aggregates {
        Some(agg) => {
            framework.register_aggregate_list_by_name(agg.split(',').collect()).unwrap_or_else(|err| panic!("Error finding aggs by name {}", err))
        }
        None => framework.register_default_aggregates().unwrap()
    };
    

    let result = framework.run().unwrap_or_else(|err| panic!("Calculating aggregates failed: {}", err));

    let mut symbols = Vec::new();
    let mut slices = Vec::new();
    let mut aggregate_names = Vec::new();
    let mut values = Vec::new();

    for row in result{
        symbols.push(row.symbol.clone());
        slices.push(row.slice);
        aggregate_names.push(row.aggregate_name.clone());
        values.push(row.value);
    }

    let df = df!("symbol" => &symbols,
             "slice" => &slices,
             "aggregate_name" => &aggregate_names,
             "value" => &values,
            ).unwrap();

    Ok(pyo3_polars::PyDataFrame(df))

}

/// A Python module implemented in Rust.
#[pymodule]
fn python(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(compute_aggregates, m)?)?;
    Ok(())
}
