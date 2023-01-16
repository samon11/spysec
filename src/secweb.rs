mod parser;
pub mod persist;
pub mod models;

use std::error::Error;
use std::sync::{Arc, Mutex};
use chrono::{NaiveDate, Datelike};
use reqwest::{RequestBuilder, Client};
use reqwest::{Url};

use parser::index::{IndexEntry, extract_index_entries, get_quarter};

use self::models::FilingTransaction;
use self::parser::FilingDoc;

const BASEURL: &str = "https://www.sec.gov/Archives/";
type Db = Arc<Mutex<Vec<FilingTransaction>>>;

pub async fn get_form(entry: &IndexEntry) -> Result<Vec<FilingTransaction>, Box<dyn Error>> {
    let url = format!("{BASEURL}{}", entry.filepath);
    println!("url: {url}");

    let client = Client::new();
    let res = client.get(
        Url::parse(&url).expect("Failed to parse valid URL")
    )
    .header("User-Agent", "Michael Samon mjsamon@icloud.com")
    .send()
    .await?;

    let body = res.text().await?;
    
    Ok(FilingDoc::new(&body))
}

pub async fn process_entries(entries: &[IndexEntry], db: Db, skip: usize, take: usize) -> Result<(), Box<dyn Error>> {
    for entry in entries.iter().cloned().skip(skip).take(take) {
        let db = db.clone();
        tokio::spawn(async move {
            let result = get_form(&entry).await;
            match result {
                Ok(mut filings) => {
                    db.lock()
                        .and_then(|mut v| Ok(v.append(&mut filings)))
                        .expect("Could not push to mutex db");
                },
                Err(err) => {
                    println!("Error occurred for filing {}: {:?}", entry.filepath, err);
                }
            }
        });
    }

    Ok(())
}

pub async fn get_daily_entries(date: NaiveDate) -> Result<Vec<IndexEntry>, Box<dyn Error>> {
    let flat_date = NaiveDate::format(&date, "%Y%m%d").to_string();
    let qtr = get_quarter(date);
    let index_url = format!(
        "https://www.sec.gov/Archives/edgar/daily-index/{}/{}/master.{}.idx"
        , date.year(), qtr, flat_date);

    let client = Client::new();
    let request = client.get(
        Url::parse(&index_url).expect("Failed to parse valid URL")
    )
    .header("User-Agent", "Michael Samon mjsamon@icloud.com");

    println!("Send request to: {index_url}");
    let body = RequestBuilder::send(request).await?.text();

    Ok(extract_index_entries(&body.await.unwrap()))
}
