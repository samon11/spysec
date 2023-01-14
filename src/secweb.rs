use std::string::String;
use std::error::Error;
use regex::Regex;


use reqwest::blocking::{RequestBuilder};
use reqwest::{Url};
use roxmltree::Document;

use crate::Filing;

pub struct SecWeb {
}

impl SecWeb {

}

pub fn get_form(url: &str) -> Result<Filing, Box<dyn Error>> {
    let client = reqwest::blocking::Client::new();
    let request = client.get(
        Url::parse(url).expect("Failed to parse valid URL")
    )
    .header("User-Agent", "Michael Samon mjsamon@icloud.com");

    println!("Send request to: {url}");
    let body = RequestBuilder::send(request)?.text()?;

    Ok(extract_filing(&body).unwrap())
}

fn extract_filing(input: &str) -> Option<Filing> {
    let pattern: Regex = Regex::new(r#"<\?xml version="1\.0"\?>[\W\S]*</ownershipDocument>"#).unwrap();

    let raw_xml = pattern.captures(input).and_then(|cap| {
        cap.iter().next().expect("Failed to parse XML").map(|m| m.as_str())
    })
    .expect("XML regex match failed");

    let doc = roxmltree::Document::parse(raw_xml).unwrap();

    let tags = doc.descendants().len();
    println!("doc size: {tags}");
    
    let mut filing = Filing::default();
    filing.shares_traded = get_tag_value(&doc, "transactionShares");
    filing.avg_price = get_tag_value(&doc, "transactionPricePerShare");
    filing.amount = filing.shares_traded * filing.avg_price;
    filing.shares_owned = get_tag_value(&doc, "sharesOwnedFollowingTransaction");
    filing.company = get_tag_text(&doc, "issuerName", false);
    filing.symbol = get_tag_text(&doc, "issuerTradingSymbol", false);
    filing.owner = get_tag_text(&doc, "rptOwnerName", false);
    filing.trans_date = get_tag_text(&doc, "transactionDate", true);

    Some(filing)
}

fn get_tag_text<'a>(doc: &'a Document<'a>, tag_name: &'a str, value_tag: bool) -> String {
    doc.descendants()
        .find(|e| e.has_tag_name(tag_name))
        .and_then(|n| {
            if value_tag {
                n.descendants().find(|v| v.has_tag_name("value"))
            } 
            else {
                Some(n)
            }
        })
        .expect("Tag name not found")
        .text()
        .ok_or("")
        .unwrap()
        .to_string()
}

fn get_tag_value(doc: &Document, tag_name: &str) -> f32 {
    let text = doc.descendants()
        .find(|e| e.has_tag_name(tag_name))
        .and_then(|c| c.descendants().find(|v| v.has_tag_name("value")))
        .expect("Value or tag name not found")
        .text();

    text.unwrap().parse::<f32>().unwrap_or(0.0)
}


