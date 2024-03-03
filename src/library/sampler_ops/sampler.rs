use polars::prelude::*;
use rand::seq::SliceRandom;

pub fn sample_dataframe(df: &DataFrame, sample_size: usize) -> Result<DataFrame, PolarsError> {
    // Shuffle the DataFrame by creating a random order for indices
    let n_rows = df.height();
    let mut rng = rand::thread_rng();
    let mut indices: Vec<u32> = (0..n_rows as u32).collect();
    indices.shuffle(&mut rng);
    let indices = UInt32Chunked::from_vec("index", indices.into_iter().take(sample_size).collect());

    // Select the rows based on shuffled indices
    let sampled_df = df.take(&indices)?;

    Ok(sampled_df)
}