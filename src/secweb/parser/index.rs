use chrono::{NaiveDate, Datelike};
use regex::Regex;
use std::string::String;

#[derive(Debug, Clone)]
pub struct IndexEntry {
    pub company_cik: String,
    pub company_name: String,
    pub form_type: String,
    pub file_date: NaiveDate,
    pub filepath: String
}

pub fn extract_index_entries(input: &str) -> Vec<IndexEntry> {
    let mut entries = Vec::<IndexEntry>::new();

    let re = Regex::new(r"\d+\|.*\|4\|.*").unwrap();
    for entry in re.find_iter(input) {
        let entry = parse_entry(entry.as_str());
        if entry.is_some() {
            entries.push(entry.unwrap());
        }
    }

    entries
}

fn parse_entry(entry: &str) -> Option<IndexEntry> {
    let values: Vec<&str> = entry.split('|').collect();

    if values.len() == 5 {
        Some(IndexEntry {
            company_cik: values[0].to_uppercase(),
            company_name: values[1].to_uppercase(),
            form_type: values[2].to_uppercase(),
            file_date: NaiveDate::parse_from_str(values[3], "%Y%m%d")
                .expect("Invalid date string"),
            filepath: values[4].to_string()
        })
    } else {
        Option::None
    }
}

pub fn get_quarter(date: NaiveDate) -> String {
    let q1 = NaiveDate::from_ymd_opt(date.year(), 1, 1).unwrap();
    let q2 = NaiveDate::from_ymd_opt(date.year(), 4, 1).unwrap();
    let q3 = NaiveDate::from_ymd_opt(date.year(), 7, 1).unwrap();
    let q4 = NaiveDate::from_ymd_opt(date.year(), 10, 1).unwrap();

    if date >= q1 && date < q2 {
        "QTR1".to_string()
    } else if date >= q2 && date < q3 {
        "QTR2".to_string()
    } else if date >= q3 && date < q4 {
        "QTR3".to_string()
    } else {
        "QTR4".to_string()
    }
}