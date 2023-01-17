use chrono::NaiveDate;
use std::{string::String};
use minidom::{Element, NSChoice};
use regex::Regex;

use crate::secweb::models::{Relationship, FilingTransaction};

#[derive(Debug, Default)]
struct XMLNode {
    text: String
} 

impl XMLNode {
    pub fn new(el: &Element) -> XMLNode {
        let mut text =  el.text().trim().to_uppercase();
        if el.has_child("value", NSChoice::Any) {
            text = el.get_child("value", NSChoice::Any).unwrap().text().trim().to_uppercase();
        }
        
        XMLNode { text: text }
    }

    pub fn parse_num(&self) -> f32 {
        self.text.parse::<f32>().unwrap_or(0.0)
    }

    pub fn parse_date(&self) -> NaiveDate {
        NaiveDate::parse_from_str(&self.text, "%Y-%m-%d").expect("Invalid date string")
    }
}

pub struct XMLFiling {
    pub transactions: Vec<FilingTransaction>,
    pub url: String
}

impl XMLFiling {
    pub fn new(url: &str) -> XMLFiling {
        XMLFiling {url: url.to_string(), transactions: Vec::<FilingTransaction>::new() }
    }

    // TODO
    // fn parse_access_num(&self) -> String {

    // }

    fn get_relationship(node: &Element) -> Vec<Relationship> {
        let mut relationships = Vec::<Relationship>::new();
    
        if Self::traverse(&node, &["reportingOwner", "reportingOwnerRelationship", "isDirector"]).unwrap_or_default().text == "1" {
            relationships.push(Relationship::DIRECTOR);
        }
    
        if Self::traverse(&node, &["reportingOwner", "reportingOwnerRelationship", "isOfficer"]).unwrap_or_default().text == "1" {
            relationships.push(Relationship::OFFICER);
        }
    
        if Self::traverse(&node, &["reportingOwner", "reportingOwnerRelationship", "isTenPercentOwner"]).unwrap_or_default().text == "1" {
            relationships.push(Relationship::TENPERC);
        }
    
        if Self::traverse(&node, &["reportingOwner", "reportingOwnerRelationship", "isOther"]).unwrap_or_default().text == "1" {
            relationships.push(Relationship::OTHER);
        }
    
        relationships
    }

    fn traverse(root: &Element, path: &[&str]) -> Option<XMLNode> {
        let mut pos = Option::None;
        let mut prev = root;

        for tag in path {
            pos = prev.get_child(tag, NSChoice::Any);
            if pos.is_some() {
                prev = pos.unwrap();
            } else {
                return Option::None;
            }
        }
        
        match pos {
            Some(el) => {
                return Some(XMLNode::new(el));
            },
            None => {
                return Option::None;
            }
        }
    }

    pub fn extract_transactions(&mut self, xml_input: &str) {
        let root: Element = xml_input.parse().unwrap();

        let company_cik = Self::traverse(&root, &["issuer", "issuerCik"]).unwrap().text;
        let rpt_owner_cik = Self::traverse(&root, &["reportingOwner", "reportingOwnerId", "rptOwnerCik"]).unwrap().text;
        let form_type = Self::traverse(&root, &["documentType"]).unwrap().text;
        let company = Self::traverse(&root, &["issuer", "issuerName"]).unwrap().text;
        let symbol =  Self::traverse(&root, &["issuer", "issuerTradingSymbol"]).unwrap().text;
        let owner = Self::traverse(&root, &["reportingOwner", "reportingOwnerId", "rptOwnerName"]).unwrap().text;
        let relationships = Self::get_relationship(&root);
        let form_date = Self::traverse(&root, &["periodOfReport"]).unwrap().parse_date();

        let table = root
            .get_child("nonDerivativeTable", NSChoice::Any)
            .expect("Filing should have a non derivative table");

        for child in table.children() {
            if child.is("nonDerivativeTransaction", NSChoice::Any) {
                let shares_traded = Self::traverse(&child, &["transactionAmounts", "transactionShares"]).unwrap().parse_num();
                let avg_price = Self::traverse(&child, &["transactionAmounts", "transactionPricePerShare"]).unwrap().parse_num();

                let filing = FilingTransaction {
                    form_url: self.url.clone(),
                    form_date: form_date.clone(),
                    company_cik:  company_cik.clone(),
                    owner_cik: rpt_owner_cik.clone(),
                    form_type: form_type.clone(),
                    company: company.clone(),
                    symbol: symbol.clone(),
                    owner: owner.clone(),
                    shares_traded: shares_traded,
                    avg_price: avg_price,
                    amount: shares_traded * avg_price,
                    shares_owned: Self::traverse(&child, &["postTransactionAmounts", "sharesOwnedFollowingTransaction"]).unwrap().parse_num(),
                    trans_date: Self::traverse(&child, &["transactionDate"]).unwrap().parse_date(),
                    relationship: relationships.clone(),
                    action_code: Self::traverse(&child, &["transactionAmounts", "transactionAcquiredDisposedCode"]).unwrap().text,
                    ownership_code: Self::traverse(&child, &["ownershipNature", "directOrIndirectOwnership"]).unwrap().text,
                    trans_code: Self::traverse(&child, &["transactionCoding", "transactionCode"]).unwrap().text
                };

                self.transactions.push(filing);
            }
        }
    }
}


