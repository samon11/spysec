pub mod secweb;

#[derive(Default, Debug)]
pub enum Relationship {
    TENPERC,
    DIRECTOR,
    OFFICER,
    #[default] OTHER,
}

#[derive(Default, Debug)]
pub enum Ownership {
    #[default] DIRECT,
    INDIRECT
}

#[derive(Default, Debug)]
pub enum ShareAction {
    #[default] ACQ,
    DISP
}


#[derive(Debug, Default)]
pub struct Filing {
    trans_date: String,
    company: String,
    symbol: String,
    owner: String,
    relationship: Vec<Relationship>,
    shares_traded: f32,
    avg_price: f32,
    amount: f32,
    shares_owned: f32,
    ownership: Ownership,
    action: ShareAction
}