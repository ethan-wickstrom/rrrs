use rand::{thread_rng};
use csv::StringRecord;
use rand::prelude::IndexedRandom;

pub struct Sampler;

impl Sampler {
    /// Samples a given number of records from the provided vector.
    ///
    /// # Arguments
    ///
    /// * `records` - A slice of `StringRecord` from which to sample.
    /// * `sample_size` - The number of records to sample.
    ///
    /// # Returns
    ///
    /// A vector containing the sampled records.
    pub fn sample(records: &[StringRecord], sample_size: usize) -> Vec<StringRecord> {
        let mut rng = thread_rng();
        let sampled_records: Vec<StringRecord> = records
            .choose_multiple(&mut rng, sample_size)
            .cloned()
            .collect();
        sampled_records
    }
}