mod csv_ops {
    pub mod csv_loader;
    pub mod csv_writer;
}
mod sampler_ops {
    pub mod sampler;
}

use csv_ops::csv_loader::CsvLoader;
use csv_ops::csv_writer::CsvWriter;
use sampler_ops::sampler::Sampler;
use std::error::Error;
use std::path::Path;


/// Performs the random sampling of rows from a CSV file and writes the output to a new file.
///
/// # Arguments
///
/// * `input_file_path` - A string slice that holds the path to the input CSV file.
/// * `output_file_path` - A string slice that holds the path to the output CSV file.
/// * `sample_size` - An optional usize that specifies the number of rows to sample.
///
/// # Returns
///
/// This function returns Result<(), Box<dyn Error>> type, where Ok(()) indicates successful execution,
/// and Err contains an error message in case of failure.
pub async fn perform_random_sampling(input_file_path: &str, output_file_path: &str, sample_size: Option<usize>) -> Result<(), Box<dyn Error>> {
    if !Path::new(input_file_path).exists() {
        return Err(From::from(format!("Input file does not exist: {}", input_file_path)));
    }

    if !Path::new(output_file_path).exists() {
        return Err(From::from(format!("Output file does not exist: {}", output_file_path)));
    }

    // Load the CSV file.
    let mut loader = CsvLoader::new(input_file_path).await?;

    // Load all records from the CSV file.
    let records = loader.load().await?;

    // Sample the records.
    let sampled_records = if let Some(size) = sample_size {
        Sampler::sample(&records, size)
    } else {
        // If no sample size is specified, default to sampling 10% of the records.
        let default_sample_size = (records.len() as f64 * 0.1).ceil() as usize;
        Sampler::sample(&records, default_sample_size)
    };

    // Write the sampled records to a new CSV file.
    CsvWriter::write(input_file_path, &sampled_records)?;

    Ok(())
}