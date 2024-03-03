use polars::prelude::*;
use parking_lot::Mutex;
use rand::{Rng, thread_rng};
use indicatif::{ProgressBar};

pub fn sample_dataframe(lazy_frame: LazyFrame, sample_size: usize, total_size: usize, pb: Arc<Mutex<ProgressBar>>) -> Result<DataFrame, PolarsError> {
    let pb = pb.lock();
    pb.set_prefix("Sampling data...");

    // Generate a random value for each row and filter based on a threshold.
    let sampled_df = lazy_frame
        .with_column(lit(thread_rng().gen_range(0.0..1.0)).alias("rand_val"))
        .filter(col("rand_val").lt(lit(sample_size as f64 / total_size as f64)))
        .drop(vec!["rand_val"]) // Drop the temporary random value column
        .collect()?; // Collects the LazyFrame into a DataFrame

    pb.set_prefix("Sampling completed");
    pb.finish_with_message("Data sampled");

    Ok(sampled_df)
}
