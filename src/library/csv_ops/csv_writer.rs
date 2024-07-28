use polars::prelude::*;
use std::error::Error;
use indicatif::ProgressBar;
use parking_lot::Mutex;
use std::fs::{self, File};
use std::path::{Path};

pub fn write_csv(df: Result<DataFrame, PolarsError>, output_file: &str, pb: Arc<Mutex<ProgressBar>>) -> Result<(), Box<dyn Error>> {
    pb.lock().set_message("Writing CSV");
    
    println!("Writing CSV to: {}", output_file);

    // Create the parent directory if it doesn't exist
    let parent_dir = Path::new(output_file).parent().ok_or_else(|| format!("Invalid output path: {}", output_file))?;
    if !parent_dir.exists() {
        fs::create_dir_all(parent_dir).map_err(|e| format!("Failed to create output directory: {}", e))?;
    }

    let file = File::create(output_file).map_err(|e| format!("Failed to create file: {}", e))?;
    let mut buffer = std::io::BufWriter::new(file);
    let df = df.map_err(|e| format!("Failed to sample data: {}", e))?;

    CsvWriter::new(&mut buffer)
        .include_header(true)
        .finish(&mut df.clone())
        .map_err(|e| format!("Failed to write CSV: {}", e))?;

    pb.lock().finish_with_message("CSV written");

    Ok(())
}