// src/library.rs
mod csv_ops {
    pub mod csv_loader;
    pub mod csv_writer;
}
mod sampler_ops {
    pub mod sampler;
}

use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;
use indicatif::{ProgressBar, ProgressStyle};
use parking_lot::Mutex;
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::time::Instant;
use csv_ops::{csv_loader, csv_writer};
use sampler_ops::sampler;

pub async fn count_rows_in_csv_stream(filepath: PathBuf, pb: Arc<Mutex<ProgressBar>>) -> Result<usize, Box<dyn std::error::Error>> {
    let file = File::open(filepath).await?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let pb_lock = pb.lock();

    pb_lock.set_message("Counting rows in CSV file...");
    pb_lock.set_prefix("Counting rows");

    drop(pb_lock);

    let start = Instant::now();

    let mut count = 0_usize;
    while lines.next_line().await?.is_some() {
        count += 1;
        pb.lock().inc(1);
        pb.lock().set_message(format!("Counting rows in CSV file... ({} rows in {:.2?})", count.clone(), start.clone().elapsed()));
    }

    let pb_lock = pb.lock();
    pb_lock.set_message(format!("Counted {} rows in CSV file in {:.2?}", count.clone(), start.clone().elapsed()));
    pb_lock.finish();

    Ok(count)
}

pub async fn perform_random_sampling(input_file: &str, output_dir: &str, sample_size: usize) -> Result<(), Box<dyn std::error::Error>> {
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
    let mut sampled_df = sampler::sample_dataframe(lf, sample_size, count_rows_in_csv_stream(input_file.parse().unwrap(), Arc::clone(&pb)).await?, Arc::clone(&pb));

    csv_writer::write_csv(&mut sampled_df, output_dir, format!("{}-{}.csv", input_file.trim_end_matches(".csv"), sample_size).as_str(), Arc::clone(&pb)).0.expect("Failed to write CSV");

    Ok(())
}