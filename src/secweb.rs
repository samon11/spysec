mod parser;

use std::error::Error;

use chrono::NaiveDate;
use reqwest::blocking::{RequestBuilder};
use reqwest::{Url};

use parser::extract_filing;

#[derive(Default, Debug)]
pub enum Relationship {
    #[default] OTHER = 1,
    TENPERC,
    DIRECTOR,
    OFFICER,
}

#[derive(Default, Debug)]
pub enum Ownership {
    #[default] DIRECT = 1,
    INDIRECT
}

#[derive(Default, Debug)]
pub enum ShareAction {
    #[default] ACQ = 1,
    DISP
}

#[derive(Debug, Default)]
pub struct Filing {
    trans_date: NaiveDate,
    company: String,
    symbol: String,
    owner: String,
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

pub fn get_form(url: &str) -> Result<Filing, Box<dyn Error>> {
    let client = reqwest::blocking::Client::new();
    let request = client.get(
        Url::parse(url).expect("Failed to parse valid URL")
    )
    .header("User-Agent", "Michael Samon mjsamon@icloud.com");

    println!("Send request to: {url}");
    let body = RequestBuilder::send(request)?.text()?;

    Ok(extract_filing(&body))
}
