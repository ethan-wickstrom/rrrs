use polars::prelude::*;
use std::error::Error;

pub async fn load_csv(filepath: &str) -> Result<DataFrame, Box<dyn Error>> {
    let df = LazyCsvReader::new(filepath)
        .has_header(true)
        .finish()
        .expect("Error reading CSV file")
        .collect();

    Ok(df?)
}