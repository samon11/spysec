use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use dotenvy::dotenv;
use std::env;

use crate::database::models::NewIssuer;
use crate::secweb::models::FilingTransaction;

use self::models::Issuer;

pub mod models;

pub fn get_connection_pool() -> Pool<ConnectionManager<PgConnection>> {
    dotenv().ok();

    let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(url);

    Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not build connection pool")
}

pub fn create_issuer(conn: &mut PgConnection, filing: &FilingTransaction) -> Issuer {
    use super::schema::issuer;

    let new_issuer = NewIssuer::map(&filing);

    diesel::insert_into(issuer::table)
        .values(&new_issuer)
        .get_result(conn)
        .expect("Should create new issuer")
}