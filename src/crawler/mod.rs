use std::{fs, sync::{Arc, Mutex, MutexGuard}, thread::{sleep, self}, time::Duration, rc::Rc};
use serde_json::Result;
use chrono::{NaiveDate, Days, Datelike};

use crate::{secweb::{models::FilingTransaction, process_entries, get_daily_entries}, database::{get_connection_pool, create_issuer, insert_models::NewIndividual, create_individual, create_form, insert_nonderiv}};

pub struct Crawler {
    current_date: NaiveDate,
}

impl Crawler {
    pub fn new(start: &NaiveDate) -> Crawler {
        Crawler { current_date: *start }
    }

    fn increment_day(&mut self)  {
        self.current_date = self.current_date
            .checked_add_days(Days::new(1))
            .unwrap();
    }

    fn get_save_dir(date: NaiveDate) -> String {
        let year = date.year();
        let month = date.format("%m");
        format!("filings/{year}/{month}")
    }

    fn save_filings_json(&self, filings: &[FilingTransaction]) {
        let date = self.current_date.format("%Y%m%d");
        let filepath = format!("{}/{date}-filing.json", Self::get_save_dir(self.current_date));
        
        fs::create_dir_all(Self::get_save_dir(self.current_date))
            .expect("Failed to create dir path");
        
        let text = serde_json::to_string(&filings).expect("Failed to serialize struct");
        fs::write(filepath, text).expect("Unable to write file");
    }

    fn save_filings_db(filings: &[FilingTransaction]) -> Result<()> {
        let pool = get_connection_pool();
        
        let total = filings.len();
        let mut i = 1;
        for trans in filings {
            let conn = &mut pool.get().unwrap();
            let issuer = create_issuer(conn, trans).ok();
            let ind = create_individual(conn, trans).ok();

            if ind.is_none() || issuer.is_none() {
                println!("failed insert {i}/{total}");
                i += 1;
                continue;
            }
            
            let form = create_form(conn, trans, &issuer.as_ref().unwrap());
            if form.is_ok() {
                let result = insert_nonderiv(
                    conn, 
                    trans, 
                    &form.as_ref().unwrap(), 
                    &issuer.unwrap(), 
                    &ind.unwrap());

                if result.is_err() {
                    println!("Error occurred adding transaction for form ID: {}", form.unwrap().form_id);
                }
            }

            println!("insert {i}/{total}");
            i += 1;
        }
        
        Ok(())
    }

    pub async fn run(&self, batch: usize) {
        if batch > 10 {
            panic!("Due to SEC limits, batch per second must be <= 10");
        }

        let db = Arc::new(Mutex::new(Vec::<FilingTransaction>::new()));

        let body = get_daily_entries(self.current_date).await.unwrap();
        
        assert_ne!(body.len(), 0);
        let second_delay = Duration::from_secs(1);
        
        let mut skip = 0;
        let total = body.len() / batch;
        for i in 0..total {
            println!("Get {i}/{total}");

            process_entries(&body, db.clone(), skip, batch).await.unwrap();
            
            skip += batch;

            // avoid SEC rate limiting by sleeping for 1 sec
            sleep(second_delay);
        }
    
        let filings = db.lock().unwrap();
        
        self.save_filings_json(&filings);

        Self::save_filings_db(&filings).expect("Should have saved to local file and db");
    }
}