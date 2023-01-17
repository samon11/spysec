pub mod index;
pub mod xmlfiling;
use regex::Regex;

use self::xmlfiling::XMLFiling;
use super::models::FilingTransaction;

pub struct FilingDoc;

impl FilingDoc {
    pub fn new(url: &str, content: &str) -> Vec<FilingTransaction> {
        let mut filing = XMLFiling::new(url);
        let content = Self::extract_xml(content);

        filing.extract_transactions(Self::extract_xml(&content).as_str());
        filing.transactions
    }

    fn extract_xml(input: &str) -> String {
        let pattern: Regex =
            Regex::new(r#"<\?xml version="1\.0"\?>[\W\S]*</ownershipDocument>"#).unwrap();
    
        let result = pattern
            .captures(input)
            .and_then(|cap| {
                cap.iter()
                    .next()
                    .expect("Failed to parse XML")
                    .map(|m| m.as_str())
            })
            .expect("XML regex match failed");

        let result = result
            .replace("<ownershipDocument>", "<ownershipDocument xmlns=\"\">");
        
        result
    }
}
