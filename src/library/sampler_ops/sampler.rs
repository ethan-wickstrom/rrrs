use polars::prelude::*;
use parking_lot::Mutex;
use indicatif::{ParallelProgressIterator, ProgressBar};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::sync::Arc;

pub async fn sample_dataframe(lazy_frame: LazyFrame, sample_size: usize, pb: Arc<Mutex<ProgressBar>>) -> Result<DataFrame, PolarsError> {
    let pb = pb.lock();
    pb.set_prefix("Sampling data...");

    let df = lazy_frame
        .collect_concurrently()?
        .fetch_blocking()
        .par_iter()
        .progress_with(pb.clone())
        .map(|df| df.clone())
        .collect::<Vec<DataFrame>>()
        .into_iter()
        .fold(DataFrame::default(), |acc, df| acc.vstack(&df).unwrap());

    let sampled_df = df.sample_n_literal(sample_size, false, true, None)?;

    pb.set_prefix("Sampling completed");
    pb.finish_with_message("Data sampled");

    Ok(sampled_df)
}

