use polars::prelude::*;
use std::error::Error;

pub async fn load_csv(filepath: &str) -> Result<DataFrame, Box<dyn Error>> {
    let df = CsvReader::from_path(filepath)?
        .infer_schema(Some(10))
        .has_header(true)
        .finish()
        .map_err(|e| e.into());

    df
}