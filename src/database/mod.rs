use bigdecimal::{BigDecimal, FromPrimitive};
use chrono::NaiveDate;
use diesel::helper_types::exists;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::result::Error;
use diesel::r2d2::Pool;
use dotenvy::dotenv;
use std::env;

use crate::database::insert_models::{NewIndividual, NewIssuer, NewForm, NewNonDerivTransaction};
use crate::database::query_models::Form;
use crate::schema::issuer;
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

pub fn create_issuer(conn: &mut PgConnection, filing: &FilingTransaction) -> Result<Issuer, Error> {
    use super::schema::issuer::dsl::*;

    let new_issuer = NewIssuer::map(&filing);
    let existing: Result<Issuer, Error> = issuer
        .filter(cik.like(new_issuer.cik))
        .first::<Issuer>(conn);

    match existing {
        Ok(result) => {
            return Ok(result);
        },
        Err(_) => {
            return diesel::insert_into(super::schema::issuer::table)
                .values(&new_issuer)
                .get_result(conn);
        }
    }
}

pub fn create_individual(conn: &mut PgConnection, filing: &FilingTransaction) -> Result<Individual, Error> {
    use super::schema::individual::dsl::*;

    let new_ind = NewIndividual::map(&filing);
    let existing: Result<Individual, Error> = individual
        .filter(cik.like(new_ind.cik))
        .first::<Individual>(conn);

    match existing {
        Ok(result) => {
            Ok(result)
        },
        Err(_) => {
            return diesel::insert_into(super::schema::individual::table)
                .values(&new_ind)
                .get_result(conn);
        }
    }
}

pub fn create_form(conn: &mut PgConnection, filing: &FilingTransaction, issuer: &Issuer) -> Result<Form, Error> {
    use super::schema::form::dsl::*;

    let new_form = NewForm::map(&filing, issuer.issuer_id);
    let existing: Result<Form, Error> = form
        .filter(url.like(&new_form.url))
        .first::<Form>(conn);

    match existing {
        Ok(result) => {
            Ok(result)
        },
        Err(_) => {
            return diesel::insert_into(super::schema::form::table)
                .values(&new_form)
                .get_result(conn);
        }
    }
}

pub fn insert_nonderiv(conn: &mut PgConnection, filing: &FilingTransaction, form: &Form, issuer: &Issuer, ind: &Individual) -> Result<NonDerivTransaction, Error> {
    use super::schema::non_deriv_transaction::dsl::*;
    
    let new_trans = NewNonDerivTransaction::map(filing, form.form_id, issuer.issuer_id, ind.individual_id);

    let balance = BigDecimal::from_f32(filing.shares_owned).unwrap();
    let existing = non_deriv_transaction
        .filter(FormId.eq(form.form_id)
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