use std::fs;
use serde_json::Result;


pub fn save_filings(filepath: &str, filings: &[String]) -> Result<()> {
    let text = serde_json::to_string(filings).expect("Failed to serialize struct");
    fs::write(filepath, text).expect("Unable to write file");
    Ok(())
}