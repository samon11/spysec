mod parser;
pub mod persist;

use std::error::Error;
use std::sync::{Arc, Mutex};
use chrono::{NaiveDate, Datelike};
use reqwest::{RequestBuilder, Client};
use reqwest::{Url};
use serde::{Serialize};

use parser::filing::extract_transactions;
use parser::index::{IndexEntry, extract_index_entries, get_quarter};

const BASEURL: &str = "https://www.sec.gov/Archives/";
type Db = Arc<Mutex<Vec<String>>>;

#[derive(Default, Debug, Serialize)]
pub enum Relationship {
    #[default] OTHER = 1,
    TENPERC,
    DIRECTOR,
    OFFICER,
}

#[derive(Default, Debug, Serialize)]
pub enum Ownership {
    #[default] DIRECT = 1,
    INDIRECT
}

#[derive(Default, Debug, Serialize)]
pub enum ShareAction {
    #[default] ACQ = 1,
    DISP
}

#[derive(Debug, Default, Serialize)]
pub struct Filing {
    trans_date: NaiveDate,
    pub company: String,
    symbol: String,
    pub owner: String,
    relationship: Vec<Relationship>,
    shares_traded: f32,
    avg_price: f32,
    amount: f32,
    shares_owned: f32,
    ownership: Ownership,
    action: ShareAction,
    company_cik: String,
    owner_cik: String,
    form_type: String
}

pub async fn get_form(entry: &IndexEntry) -> Result<String, Box<dyn Error>> {
    let url = BASEURL.to_string() + &entry.filepath;
    println!("url: {url}");

    let client = Client::new();
    let res = client.get(
        Url::parse(&url).expect("Failed to parse valid URL")
    )
    .header("User-Agent", "Michael Samon mjsamon@icloud.com")
    .send()
    .await?;

    let body = res.text().await?;
    let filing = extract_transactions(&body);
    
    Ok(filing)
}

pub async fn process_entries(entries: &[IndexEntry], db: Db, skip: usize, take: usize) -> Result<(), Box<dyn Error>> {
    for entry in entries.iter().cloned().skip(skip).take(take) {
        let db = db.clone();
        tokio::spawn(async move {
            let result = get_form(&entry).await;
            match result {
                Ok(filing) => {
                    db.lock()
                        .and_then(|mut v| Ok(v.push(filing)))
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
