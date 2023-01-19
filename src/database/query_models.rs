use chrono::NaiveDate;
use diesel::prelude::*;
use bigdecimal::BigDecimal;

#[derive(Queryable, Debug)]
pub struct Form {
    pub form_id: i64,
    pub issuer_id: i32,
    pub date_reported: NaiveDate,
    pub form_type: String,
    pub txt_url: String,
    pub access_no: String,
    pub web_url: String
}

#[derive(Queryable, Debug)]
pub struct Individual {
    pub individual_id: i32,
    pub cik: String,
    pub full_name: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>
}

#[derive(Queryable, Debug)]
pub struct Issuer {
    pub issuer_id: i32,
    pub name: String,
    pub symbol: String,
    pub cik: String
}

#[derive(Queryable, Debug)]
pub struct NonDerivTransaction {
    pub transaction_id: i64,
    pub date_reported: NaiveDate,
    pub form_id: i64,
    pub issuer_id: i32,
    pub individual_id: i32,
    pub action_code: Option<String>,
    pub ownership_code: Option<String>,
    pub transaction_code: Option<String>,
    pub shares_balance: BigDecimal,
    pub shares_traded: BigDecimal,
    pub avg_price: BigDecimal,
    pub amount: BigDecimal,
    pub relationships: Vec<i32>
}