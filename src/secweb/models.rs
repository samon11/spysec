use serde::{Serialize};
use chrono::NaiveDate;

#[derive(Default, Debug, Serialize, Clone, Copy)]
pub enum Relationship {
    #[default] OTHER = 1,
    TENPERC,
    DIRECTOR,
    OFFICER,
}

#[derive(Debug, Default, Serialize, Clone)]
pub struct FilingTransaction {
    pub trans_date: NaiveDate,
    pub form_date: NaiveDate,
    pub company: String,
    pub symbol: String,
    pub owner: String,
    pub relationship: Vec<Relationship>,
    pub shares_traded: f32,
    pub avg_price: f32,
    pub amount: f32,
    pub shares_owned: f32,
    pub trans_code: String,
    pub ownership_code: String,
    pub action_code: String,
    pub company_cik: String,
    pub owner_cik: String,
    pub form_type: String,
    pub form_url: String,
    pub access_no: String,
}