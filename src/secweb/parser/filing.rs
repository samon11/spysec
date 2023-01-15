use regex::Regex;
use std::string::String;
use roxmltree::{Document, Node};

use super::{Relationship, ShareAction, Ownership};

pub fn extract_transactions(input: &str) -> String {
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

    return raw_xml.to_string();

    // let root = roxmltree::Document::parse(raw_xml).unwrap();

    // let tags = root.descendants().len();
    // println!("doc size: {tags}");
    
    // let table = root.
    //     descendants()
    //     .find(|n| n.has_tag_name("nonDerivativeTransaction"))
    //     .and_then(|n| Some(n))
    //     .unwrap();

    // for child in table.descendants() {
    //     let mut filing = Filing::default();
    //     filing.shares_traded = get_node_value(child, "transactionShares").unwrap_or(0.0);
    //     filing.avg_price = get_node_value(child, "transactionPricePerShare").unwrap_or(0.0);
    //     filing.amount = filing.shares_traded * filing.avg_price;
    //     filing.shares_owned = get_node_value(child, "sharesOwnedFollowingTransaction").unwrap_or(0.0);
    //     filing.company = get_tag_text(&root, "issuerName", false);
    //     filing.symbol = get_tag_text(&root, "issuerTradingSymbol", false);
    //     filing.owner = get_tag_text(&root, "rptOwnerName", false);
    //     filing.relationship = get_relationship(child);
    //     filing.action = get_action(child);
    //     filing.ownership = get_ownership(child);
    //     filing.company_cik = get_tag_text(&root, "issuerCik", false);
    //     filing.owner_cik = get_tag_text(&root, "rptOwnerCik", false);
    //     filing.form_type = get_tag_text(&root, "documentType", false);
    
    //     let trans_date = get_node_text(child, "transactionDate").unwrap();
    //     filing.trans_date = NaiveDate::parse_from_str(&trans_date, "%Y-%m-%d")
    //         .expect("Could not parse date string");

    //     filings.push(filing);
    // }

}

fn get_tag_text(doc: &Document, tag_name: &str, value_tag: bool) -> String {
    doc.descendants()
        .find(|e| e.has_tag_name(tag_name))
        .and_then(|n| {
            if value_tag {
                n.descendants().find(|v| v.has_tag_name("value"))
            } else {
                Some(n)
            }
        })
        .expect(format!("Tag name not found {tag_name}").as_str())
        .text()
        .unwrap()
        .to_string()
        .to_uppercase()
}

fn get_node_text(node: Node, tag_name: &str) -> Option<String> {
    node
        .children()
        .find(|c| c.has_tag_name(tag_name))
        .and_then(|c| c.children()
        .find(|c| c.has_tag_name("value")))
        .and_then(|c| c.text())
        .and_then(|c| Some(c.to_string()))
}

fn get_node_value(node: Node, tag_name: &str) -> Option<f32> {
    node
        .children()
        .find(|c| c.has_tag_name(tag_name))
        .and_then(|c| c.children()
        .find(|n| n.has_tag_name("value")))
        .and_then(|c| c.text())
        .and_then(|c| c.parse::<f32>().ok())
}

fn get_tag_value(doc: &Document, tag_name: &str) -> Option<f32> {
    let text = doc
        .descendants()
        .find(|e| e.has_tag_name(tag_name))
        .and_then(|c| c.descendants().find(|v| v.has_tag_name("value")))
        .expect(format!("Value or tag name not found {tag_name}").as_str())
        .text();

    text.unwrap().parse::<f32>().ok()
}

fn get_relationship(node: Node) -> Vec<Relationship> {
    let mut relationships = Vec::<Relationship>::new();

    if get_node_text(node, "isDirector").unwrap() == "1" {
        relationships.push(Relationship::DIRECTOR);
    }

    if get_node_text(node, "isOfficer").unwrap() == "1" {
        relationships.push(Relationship::OFFICER);
    }

    if get_node_text(node, "isTenPercentOwner").unwrap() == "1" {
        relationships.push(Relationship::TENPERC);
    }

    if get_node_text(node, "isOther").unwrap() == "1" {
        relationships.push(Relationship::OTHER);
    }

    relationships
}

fn get_action(node: Node) -> ShareAction {
    let _value = node.descendants()
        .find(|v| v.has_tag_name("transactionAcquiredDisposedCode"))
        .and_then(|c| c.descendants().find(|v| v.has_tag_name("value")))
        .and_then(|n| n.text());

    if let Some(_value) = Some("D") {
        ShareAction::DISP
    } else if let Some(_value) = Some("A") {
        ShareAction::ACQ
    } else {
        panic!("Transaction code could not be determined");
    }
}

fn get_ownership(node: Node) -> Ownership {
    let ownership = get_node_text(node, "directOrIndirectOwnership").unwrap();
    if ownership == "D" {
        Ownership::DIRECT
    } else if ownership == "I" {
        Ownership::INDIRECT
    } else {
        panic!("Ownership code could not be determined");
    }
}