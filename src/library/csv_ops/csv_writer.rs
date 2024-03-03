use polars::prelude::*;
use std::error::Error;
use std::fs::File;
use indicatif::ProgressBar;
use parking_lot::Mutex;

pub fn write_csv(df: Result<DataFrame, PolarsError>, output_dir: &str, file_name: &str, pb: Arc<Mutex<ProgressBar>>) -> (Result<(), Box<dyn Error>>, ()) {
    pb.lock().set_message("Writing CSV");
    let file = File::create(format!("{}/{}", output_dir, file_name)).unwrap();
    let mut buffer = std::io::BufWriter::new(file);
    let df = df.expect("Failed to sample data");

    // No need to clone df, we can directly pass the reference
    (CsvWriter::new(&mut buffer)
        .include_header(true)
        .finish(&mut df.clone())
        .map_err(|e| e.into()), pb.lock().finish_with_message("CSV written"))
}