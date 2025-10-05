// src/data.rs

use std::fs;
use std::path::Path;
use std::error::Error;
use std::collections::HashMap;
use serde::Deserialize;

use crate::types::Candle;

pub fn load_token_csvs(
    folder: &str,
    token_prefix: &str,
) -> Result<HashMap<String, Vec<Candle>>, Box<dyn Error>> {
    let mut result = HashMap::new();
    let entries = fs::read_dir(folder)?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if let Some(filename) = path.file_name().and_then(|f| f.to_str()) {
            if filename.to_lowercase().starts_with(&token_prefix.to_lowercase()) &&
               filename.ends_with(".csv") {

                let file = fs::File::open(&path)?;
                let mut rdr = csv::ReaderBuilder::new()
                    .has_headers(false)
                    .from_reader(file);

                let mut candles = Vec::new();
                for result in rdr.deserialize() {
                    let record: Candle = result?;
                    candles.push(record);
                }

                result.insert(filename.to_string(), candles);
            }
        }
    }

    Ok(result)
}
