use polars::prelude::*;
use std::error::Error;
use std::fs::File;

pub fn write_csv(df: &mut Result<DataFrame, PolarsError>, output_dir: &str, file_name: &str) -> Result<(), Box<dyn Error>> {
    let file = File::create(format!("{}/{}", output_dir, file_name))?;
    let mut buffer = std::io::BufWriter::new(file);
    let df = df.as_mut().unwrap();

    // No need to clone df, we can directly pass the reference
    CsvWriter::new(&mut buffer)
        .include_header(true)
        .finish(df)
        .map_err(|e| e.into())
}