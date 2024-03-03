// src/library.rs
mod csv_ops {
    pub mod csv_loader;
    pub mod csv_writer;
}
mod sampler_ops {
    pub mod sampler;
}

use indicatif::{ProgressBar, ProgressStyle};
use csv_ops::{csv_loader, csv_writer};
use sampler_ops::sampler;


pub async fn perform_random_sampling(input_file: &str, output_dir: &str, sample_size: usize) -> Result<(), Box<dyn std::error::Error>> {
    let pb = ProgressBar::new_spinner();
    pb.set_style(ProgressStyle::default_spinner());

    let df = csv_loader::load_csv(input_file).await?;
    pb.set_message("Sampling...");

    // Adjust sampler usage to propagate the error correctly
    let mut sampled_df = sampler::sample_dataframe(&df, sample_size);

    pb.set_message("Writing CSV...");
    // Now `sampled_df` is correctly unwrapped and passed as a reference    
    csv_writer::write_csv(&mut sampled_df, output_dir, format!("{}-{}.csv", input_file.trim_end_matches(".csv"), sample_size).as_str())?;

    pb.finish_with_message("Completed");

    Ok(())
}