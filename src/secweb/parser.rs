use chrono::NaiveDate;
use regex::Regex;
use std::string::String;
use roxmltree::Document;

use super::{Filing, Relationship, ShareAction, Ownership};

pub fn extract_filing(input: &str) -> Filing {
    let pattern: Regex =
        Regex::new(r#"<\?xml version="1\.0"\?>[\W\S]*</ownershipDocument>"#).unwrap();

    let raw_xml = pattern
        .captures(input)
        .and_then(|cap| {
            cap.iter()
                .next()
                .expect("Failed to parse XML")
                .map(|m| m.as_str())
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
    filing.relationship = get_relationship(&doc);
    filing.action = get_action(&doc);
    filing.ownership = get_ownership(&doc);
    filing.company_cik = get_tag_text(&doc, "issuerCik", false);
    filing.owner_cik = get_tag_text(&doc, "rptOwnerCik", false);
    filing.form_type = get_tag_text(&doc, "documentType", false);

    let trans_date = get_tag_text(&doc, "transactionDate", true);
    filing.trans_date = NaiveDate::parse_from_str(&trans_date, "%Y-%m-%d")
        .expect("Could not parse date string");

    filing
}

fn get_tag_text<'a>(doc: &'a Document<'a>, tag_name: &'a str, value_tag: bool) -> String {
    doc.descendants()
        .find(|e| e.has_tag_name(tag_name))
        .and_then(|n| {
            if value_tag {
                n.descendants().find(|v| v.has_tag_name("value"))
            } else {
                Some(n)
            }
        })
        .expect("Tag name not found")
        .text()
        .unwrap()
        .to_string()
        .to_uppercase()
}

fn get_tag_value(doc: &Document, tag_name: &str) -> f32 {
    let text = doc
        .descendants()
        .find(|e| e.has_tag_name(tag_name))
        .and_then(|c| c.descendants().find(|v| v.has_tag_name("value")))
        .expect("Value or tag name not found")
        .text();

    text.unwrap().parse::<f32>().unwrap_or(0.0)
}

fn get_relationship(doc: &Document) -> Vec<Relationship> {
    let mut relationships = Vec::<Relationship>::new();

    if get_tag_text(doc, "isDirector", false) == "1" {
        relationships.push(Relationship::DIRECTOR);
    }

    if get_tag_text(doc, "isOfficer", false) == "1" {
        relationships.push(Relationship::OFFICER);
    }

    if get_tag_text(doc, "isTenPercentOwner", false) == "1" {
        relationships.push(Relationship::TENPERC);
    }

    if get_tag_text(doc, "isOther", false) == "1" {
        relationships.push(Relationship::OTHER);
    }

    relationships
}

fn get_action(doc: &Document) -> ShareAction {
    let _value = doc.descendants()
        .find(|n| n.has_tag_name("nonDerivativeTable"))
        .and_then(|d| d.descendants().find(|v| v.has_tag_name("transactionAcquiredDisposedCode")))
        .and_then(|c| c.descendants().find(|v| v.has_tag_name("value")))
        .expect("Value or tag name not found")
        .text();

    if let Some(_value) = Some("D") {
        ShareAction::DISP
    } else if let Some(_value) = Some("A") {
        ShareAction::ACQ
    } else {
        panic!("Transaction code could not be determined");
    }
}

fn get_ownership(doc: &Document) -> Ownership {
    let ownership = get_tag_text(doc, "directOrIndirectOwnership", true);
    if ownership == "D" {
        Ownership::DIRECT
    } else if ownership == "I" {
        Ownership::INDIRECT
    } else {
        panic!("Ownership code could not be determined");
    }
}