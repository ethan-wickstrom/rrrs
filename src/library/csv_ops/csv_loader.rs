use csv::{Reader, ReaderBuilder, StringRecord};
use std::error::Error;
use std::io;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

pub struct CsvLoader {
    reader: Reader<io::Cursor<Vec<u8>>>,
}

impl CsvLoader {
    pub async fn new(file_path: &str) -> Result<CsvLoader, Box<dyn Error>> {
        let mut file = File::open(file_path).await?;
        let mut contents = vec![];
        file.read_to_end(&mut contents).await?;

        let reader = ReaderBuilder::new().from_reader(io::Cursor::new(contents));

        Ok(CsvLoader { reader })
    }

    pub async fn load(&mut self) -> Result<Vec<StringRecord>, Box<dyn Error>> {
        let mut records = Vec::new();
        for result in self.reader.records() {
            let record = result?;
            records.push(record);
        }
        Ok(records)
    }
}