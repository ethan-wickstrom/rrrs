// src/library.rs
mod csv_ops {
    pub mod csv_loader;
    pub mod csv_writer;
}
mod sampler_ops {
    pub mod sampler;
}

use std::sync::Arc;
use std::time::Duration;
use indicatif::{ProgressBar, ProgressStyle};
use parking_lot::Mutex;
use csv_ops::{csv_loader, csv_writer};
use sampler_ops::sampler;

pub async fn perform_random_sampling(input_file: &str, output_file: &str, sample_size: usize) -> Result<(), Box<dyn std::error::Error>> {
    let pb = ProgressBar::new_spinner();

    let pb = Arc::new(Mutex::new(pb));

    pb.lock()
        .set_style(ProgressStyle::default_spinner()
            .template("{msg}\n{prefix:.bold} {spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
            .unwrap()
            .progress_chars("#>-")
        );

    pb.lock().enable_steady_tick(Duration::from_millis(100));

    pb.lock().set_message("Initializing...");

    let lf = csv_loader::load_csv(input_file, Arc::clone(&pb)).await?;

    // Adjust sampler usage to propagate the error correctly
    let sampled_df = sampler::sample_dataframe(lf, sample_size, Arc::clone(&pb)).await;
    
    csv_writer::write_csv(sampled_df, output_file, Arc::clone(&pb))?;

    Ok(())
}