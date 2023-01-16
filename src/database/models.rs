use chrono::NaiveDate;
use diesel::prelude::*;
use crate::schema::issuer;

use crate::secweb::models::FilingTransaction;

#[derive(Queryable)]
pub struct Form {
    pub form_id: i64,
    pub issuer_id: i32,
    pub date_reported: NaiveDate,
    pub form_type: String
}

#[derive(Queryable)]
pub struct Individual {
    pub individual_id: i32,
    pub cik: String,
    pub full_name: String,
    pub first_name: String,
    pub last_name: String
}

#[derive(Queryable)]
pub struct Issuer {
    pub issuer_id: i32,
    pub name: String,
    pub symbol: String,
    pub cik: String
}

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
        NewIssuer { issuer_name: &filing.company, issuer_symbol: &filing.symbol, cik: &filing.company_cik }
    }
}

#[derive(Queryable)]
pub struct NonDerivTransaction {
    pub transaction_id: u64,
    pub date_reported: NaiveDate,
    pub form_id: u32,
    pub issuer_id: u32,
    pub individual_id: u32,
    pub action_code: String,
    pub ownership_code: String,
    pub transaction_code: String,
    pub shares_balance: f32,
    pub shares_traded: f32,
    pub avg_price: f32,
    pub amount: f32,
    pub relationships: Vec<u8>
}