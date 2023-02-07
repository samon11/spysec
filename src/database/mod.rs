use bigdecimal::{BigDecimal, FromPrimitive};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::result::Error;
use diesel::r2d2::Pool;
use dotenvy::dotenv;
use std::collections::HashMap;
use std::env;
use std::sync::{Arc, Mutex};

use crate::database::insert_models::{NewIndividual, NewIssuer, NewForm, NewNonDerivTransaction};
use crate::database::query_models::Form;
use crate::secweb::models::FilingTransaction;

pub mod query_models;
pub mod insert_models;

use self::query_models::{Issuer, Individual, NonDerivTransaction};

pub fn get_connection_pool() -> Pool<ConnectionManager<PgConnection>> {
    dotenv().ok();

    let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(url);

    Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not build connection pool")
}

pub struct SqlHelper{
    issuers_cache: Arc<Mutex<HashMap<String, i32>>>,
    form_cache: Arc<Mutex<HashMap<String, i64>>>,
    ind_cache: Arc<Mutex<HashMap<String, i32>>>
}

impl SqlHelper {
    pub fn new() -> SqlHelper {
        SqlHelper { 
            issuers_cache: Arc::new(Mutex::new(HashMap::new())) ,
            form_cache: Arc::new(Mutex::new(HashMap::new())), 
            ind_cache: Arc::new(Mutex::new(HashMap::new())) }
    }

    pub fn create_issuer(&mut self, conn: &mut PgConnection, filing: &FilingTransaction) -> Result<i32, Error> {
        use super::schema::issuer::dsl::*;
        
        let new_issuer = NewIssuer::map(&filing);
        
        let cache = &mut self.issuers_cache.lock().unwrap();
        let cache_item = cache.get(&new_issuer.cik.to_string());
        
        if cache_item.is_some()  {
            return Ok(*cache_item.unwrap())
        }
    
        let existing: Result<Issuer, Error> = issuer
            .filter(cik.like(new_issuer.cik))
            .first::<Issuer>(conn);
        
        match existing {
            Ok(result) => {
                cache.insert(result.cik, result.issuer_id);
                return Ok(result.issuer_id);
            },
            Err(_) => {
                let new_issuer: Result<Issuer, Error> = 
                    diesel::insert_into(super::schema::issuer::table)
                    .values(&new_issuer)
                    .get_result(conn);

                if !new_issuer.is_err() {
                    let new_issuer = new_issuer.unwrap();
                    cache.insert(new_issuer.cik, new_issuer.issuer_id);
                    Ok(new_issuer.issuer_id)
                } else {
                    Err(new_issuer.unwrap_err())
                }
            }
        }

    }
    
    pub fn create_individual(&mut self, conn: &mut PgConnection, filing: &FilingTransaction) -> Result<i32, Error> {
        use super::schema::individual::dsl::*;
        
        let new_ind = NewIndividual::map(&filing);

        let mut cache = self.ind_cache.lock().unwrap();
        let cache_item = cache.get(&new_ind.cik.to_string());
        
        if cache_item.is_some()  {
            return Ok(*cache_item.unwrap());
        }

        let existing: Result<Individual, Error> = individual
            .filter(cik.like(new_ind.cik))
            .first::<Individual>(conn);
        
        match existing {
            Ok(result) => {
                cache.insert(result.cik, result.individual_id);
                Ok(result.individual_id)
            },
            Err(_) => {
                let new_ind: Result<Individual, Error> = 
                    diesel::insert_into(super::schema::individual::table)
                    .values(&new_ind)
                    .get_result(conn);

                if !new_ind.is_err() {
                    let new_ind = new_ind.unwrap();
                    cache.insert(new_ind.cik, new_ind.individual_id);
                    Ok(new_ind.individual_id)
                } else {
                    Err(new_ind.unwrap_err())
                }
            }
        }
    }
    
    pub fn create_form(&self, conn: &mut PgConnection, filing: &FilingTransaction, issuer_id: i32) -> Result<i64, Error> {
        use super::schema::form::dsl::*;
        
        let new_form = NewForm::map(&filing, issuer_id);

        let cache = &mut self.form_cache.lock().unwrap();
        let cache_item = cache.get(&new_form.access_no);
        
        if cache_item.is_some()  {
            return Ok(*cache_item.unwrap());
        }

        let existing: Result<Form, Error> = form
            .filter(AccessNo.like(&new_form.access_no))
            .first::<Form>(conn);
        
        match existing {
            Ok(result) => {
                cache.insert(result.access_no, result.form_id);
                Ok(result.form_id)
            },
            Err(_) => {
                let new_form: Result<Form, Error> = 
                    diesel::insert_into(super::schema::form::table)
                    .values(&new_form)
                    .get_result(conn);

                    
                if !new_form.is_err() {
                    let new_form = new_form.unwrap();
                    cache.insert(new_form.access_no, new_form.form_id);
                    Ok(new_form.form_id)
                } else {
                    Err(new_form.unwrap_err())
                }
            }
        }
    }
    
    pub fn insert_nonderiv(conn: &mut PgConnection, filing: &FilingTransaction, form_id: i64, issuer_id: i32, ind_id: i32) -> Result<NonDerivTransaction, Error> {
        use super::schema::non_deriv_transaction::dsl::*;
        
        let new_trans = NewNonDerivTransaction::map(filing, form_id, issuer_id, ind_id);
        
        let balance = BigDecimal::from_f32(filing.shares_owned).unwrap();
        let existing = non_deriv_transaction
            .filter(FormId.eq(form_id)
            .and(DateReported.eq(filing.trans_date)
            .and(SharesBalance.eq(balance))))
            .first::<NonDerivTransaction>(conn);

    match existing {
        Ok(result) => {
            Ok(result)
        },
        Err(_) => {
            return diesel::insert_into(super::schema::non_deriv_transaction::table)
                .values(&new_trans)
                .get_result(conn);
            }
        }
    }

    pub fn bulk_insert_nonderivs(conn: &mut PgConnection, transactions: &[NewNonDerivTransaction]) -> Result<usize, Error> {
        use super::schema::non_deriv_transaction;

        diesel::insert_into(non_deriv_transaction::table)
            .values(transactions)
            .execute(conn)
    }
}