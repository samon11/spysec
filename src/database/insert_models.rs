use bigdecimal::{BigDecimal, FromPrimitive};
use chrono::NaiveDate;
use diesel::prelude::*;
use crate::{schema::*, secweb::models::Relationship};
use diesel::pg::data_types::PgNumeric;

use crate::secweb::models::FilingTransaction;

use super::query_models::Issuer;

#[derive(Insertable)]
#[diesel(table_name = issuer)]
pub struct NewIssuer<'a> {
    #[diesel(column_name = "Name")]
    pub issuer_name: &'a str,

    #[diesel(column_name = "Symbol")]
    pub issuer_symbol: &'a str,

    pub cik: &'a str
}

impl NewIssuer<'_> {
    pub fn map(filing: &FilingTransaction) -> NewIssuer {
        NewIssuer { 
            issuer_name: &filing.company, 
            issuer_symbol: &filing.symbol, 
            cik: &filing.company_cik 
        }
    }
}

#[derive(Insertable)]
#[diesel(table_name = individual)]
pub struct NewIndividual<'a> {
    #[diesel(column_name = "FullName")]
    pub full_name: String,

    pub cik: &'a str,

    #[diesel(column_name = "FirstName")]
    pub first_name: Option<String>,

    #[diesel(column_name = "LastName")]
    pub last_name: Option<String>,
}

impl NewIndividual<'_> {
    pub fn map(filing: &FilingTransaction) -> NewIndividual {
        let split: Vec<_> = filing.owner.split(" ")
            .map(|c| c.to_string())
            .collect();

        // TODO: distinguish between people and company names
        if split.len() >= 2 {
            let mut owner: Vec<_> = filing.owner.split(" ").collect();
            owner.reverse();

            let split = split.clone();
            let last_name = Some(split[0].clone());
            let first_name = Some(split[1..split.len()].join(" "));
    
            return NewIndividual {
                full_name: filing.owner.to_string(), 
                cik: &filing.owner_cik, 
                first_name: first_name, 
                last_name: last_name };
        } else {
            return NewIndividual {
                full_name: filing.owner.to_string(),
                cik: &filing.owner_cik,
                first_name: Option::None,
                last_name: Option::None
            }
        }
    }
}

#[derive(Insertable)]
#[diesel(table_name = form)]
pub struct NewForm {
    #[diesel(column_name = "IssuerId")]
    pub issuer_id: i32,

    #[diesel(column_name = "DateReported")]
    pub date_reported: NaiveDate,

    #[diesel(column_name = "FormType")]
    pub form_type: String,

    pub url: String,

    #[diesel(column_name = "AccessNo")]
    pub access_no: String
}

impl NewForm {
    pub fn map(filing: &FilingTransaction, issuer_id: i32) -> NewForm {
        NewForm { 
            issuer_id: issuer_id, 
            date_reported: filing.form_date, 
            form_type: filing.form_type.to_string(),
            url: filing.form_url.to_string(),
            access_no: filing.access_no.to_string()
        }
    }
}

#[derive(Insertable)]
#[diesel(table_name = non_deriv_transaction)]
pub struct NewNonDerivTransaction {
    #[diesel(column_name = "DateReported")]
    pub date_reported: NaiveDate,

    #[diesel(column_name = "FormId")]
    pub form_id: i64,

    #[diesel(column_name = "IssuerId")]
    pub issuer_id: i32,

    #[diesel(column_name = "IndividualId")]
    pub individual_id: i32,

    #[diesel(column_name = "ActionCode")]
    pub action_code: Option<String>,

    #[diesel(column_name = "OwnershipCode")]
    pub ownership_code: Option<String>,

    #[diesel(column_name = "TransactionCode")]
    pub transaction_code: Option<String>,

    #[diesel(column_name = "SharesBalance")]
    pub shares_balance: BigDecimal,

    #[diesel(column_name = "SharesTraded")]
    pub shares_traded: BigDecimal,

    #[diesel(column_name = "AvgPrice")]
    pub avg_price: BigDecimal,

    #[diesel(column_name = "Amount")]
    pub amount: BigDecimal,

    #[diesel(column_name = "Relationships")]
    pub relationships: Vec<i32>
}

impl NewNonDerivTransaction {
    pub fn map(
        filing: &FilingTransaction, 
        form_id: i64, 
        issuer_id: i32, 
        individual_id: i32) -> NewNonDerivTransaction 
    {
        let relationships = filing.relationship.iter()
            .map(|r| *r as i32)
            .collect();

        NewNonDerivTransaction { 
            date_reported: filing.trans_date, 
            form_id: form_id, 
            issuer_id: issuer_id,
            individual_id: individual_id, 
            action_code: Some(filing.action_code.clone()), 
            ownership_code: Some(filing.ownership_code.clone()), 
            transaction_code: Some(filing.trans_code.clone()), 
            shares_balance: BigDecimal::from_f32(filing.shares_owned).unwrap(), 
            shares_traded: BigDecimal::from_f32(filing.shares_traded).unwrap(),
            avg_price: BigDecimal::from_f32(filing.avg_price).unwrap(), 
            amount: BigDecimal::from_f32(filing.amount).unwrap(), 
            relationships: relationships }
    }
}