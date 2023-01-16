use serde::{Serialize};
use chrono::NaiveDate;

#[derive(Default, Debug, Serialize, Clone, Copy)]
pub enum Relationship {
    #[default] OTHER = 1,
    TENPERC,
    DIRECTOR,
    OFFICER,
}

#[derive(Default, Debug, Serialize)]
pub enum Ownership {
    #[default] DIRECT = 1,
    INDIRECT
}

#[derive(Default, Debug, Serialize)]
pub enum ShareAction {
    #[default] ACQ = 1,
    DISP
}

#[derive(Debug, Default, Serialize)]
pub struct FilingTransaction {
    pub trans_date: NaiveDate,
    pub company: String,
    pub symbol: String,
    pub owner: String,
    pub relationship: Vec<Relationship>,
    pub shares_traded: f32,
    pub avg_price: f32,
    pub amount: f32,
    pub shares_owned: f32,
    pub ownership: Ownership,
    pub action: ShareAction,
    pub company_cik: String,
    pub owner_cik: String,
    pub form_type: String
}