use polars::prelude::*;
use indicatif::{ProgressBar};
use parking_lot::Mutex;
use std::sync::Arc;

pub async fn load_csv(filepath: &str, pb: Arc<Mutex<ProgressBar>>) -> Result<LazyFrame, PolarsError> {
    let pb = pb.lock();
    
    pb.set_prefix("Loading CSV...");
    pb.set_message("Reading CSV file");
    
    let reader = LazyCsvReader::new(filepath)
        .with_cache(true)
        .has_header(true)
        .finish()?;
    
    pb.set_prefix("CSV loaded");
    pb.finish_with_message("CSV processing completed");
    
    Ok(reader)
}
