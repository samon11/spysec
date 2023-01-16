use std::fs;
use serde_json::Result;

use super::models::FilingTransaction;

pub fn save_filings(filepath: &str, filings: &[FilingTransaction]) -> Result<()> {
    let text = serde_json::to_string(filings).expect("Failed to serialize struct");
    fs::write(filepath, text).expect("Unable to write file");
    Ok(())
}