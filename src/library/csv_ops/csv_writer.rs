use csv::Writer;
use std::error::Error;
use std::fs::File;
use csv::StringRecord;

pub struct CsvWriter;

impl CsvWriter {
    pub fn write(file_path: &str, records: &[StringRecord]) -> Result<(), Box<dyn Error>> {
        // First, strip the file extension from the file path.
        let file_path = file_path.trim_end_matches(".csv");
        
        
        // Rename file_path to file_path + "-" + # of records + ".csv"
        let file_path = format!("{}-{}.csv", file_path, records.len());

        let file = File::create(file_path)?;
        
        let mut writer = Writer::from_writer(file);

        for record in records {
            writer.write_record(record)?;
        }

        writer.flush()?;
        Ok(())
    }
}
