use std::{
    fs::{self, File}, 
    sync::{Arc, Mutex, }, 
    thread::{sleep}, 
    time::Duration, 
    ops::AddAssign, 
    path::Path, 
    io::BufReader};
use serde_json::Result;
use chrono::{NaiveDate, Days, Datelike};
use chrono_tz::{US::Eastern};
use futures::*;

use crate::{secweb::{models::FilingTransaction, process_entries, get_daily_entries}, database::{get_connection_pool, SqlHelper}};

pub struct Crawler {
    pub crawl_date: NaiveDate,
}

impl Crawler {
    pub fn new(start: &NaiveDate) -> Crawler {
        Crawler { crawl_date: *start }
    }

    fn yesterday() -> NaiveDate {
        chrono::Local::now()
            .with_timezone(&Eastern)
            .date_naive()
            .checked_sub_days(Days::new(1))
            .unwrap()
    }

    fn increment_day(&mut self) {
        self.crawl_date = self.crawl_date
            .checked_add_days(Days::new(1))
            .unwrap();
    }

    fn get_save_dir(date: NaiveDate) -> String {
        let year = date.year();
        let month = date.format("%m");
        format!("filings/{year}/{month}")
    }

    fn get_file_path(&self) -> String {
        let date = self.crawl_date.format("%Y%m%d");
        format!("{}/{date}-filing.json", Self::get_save_dir(self.crawl_date))
    }

    fn save_filings_json(&self, filings: &[FilingTransaction]) {
        let filepath = self.get_file_path();
        
        fs::create_dir_all(Self::get_save_dir(self.crawl_date))
            .expect("Failed to create dir path");
        
        let text = serde_json::to_string(&filings).expect("Failed to serialize struct");
        fs::write(filepath, text).expect("Unable to write file");
    }

    async fn save_filings_db(filings: &[FilingTransaction]) -> Result<()> {
        let pool = get_connection_pool();
        let helper = Arc::new(Mutex::new(SqlHelper::new()));
        
        let total = filings.len();
        let i = Arc::new(Mutex::new(0));
        let stream = stream::iter(filings);

        stream
            .for_each_concurrent(10, |trans| async {
                let conn = &mut pool.get().unwrap();
                let mut helper = helper.lock().unwrap();
                let issuer = helper.create_issuer(conn, trans).ok();
                let ind = helper.create_individual(conn, trans).ok();
                
                if ind.is_none() || issuer.is_none() {
                    let mut progress = i.lock().unwrap();
                    progress.add_assign(1);
                    println!("failed insert {progress}/{total}");
                }
                
                let form_id = helper.create_form(conn, trans, issuer.unwrap());
                if form_id.is_ok() {
                    let form_id = form_id.unwrap();
                    let result = SqlHelper::insert_nonderiv(
                        conn, 
                        trans, 
                        form_id, 
                        issuer.unwrap(), 
                        ind.unwrap());
    
                    if result.is_err() {
                        println!("Error occurred adding transaction for form ID: {}", form_id);
                    }
                }

                let mut progress = i.lock().unwrap();
                progress.add_assign(1);
                println!("insert {progress}/{total}");
            }).await;
        
        Ok(())
    }

    pub async fn run(&mut self, batch: usize) {
        if batch > 10 {
            panic!("Due to SEC limits, batch per second must be <= 10");
        }

        if self.crawl_date > Self::yesterday() {
            println!("{} is today... waiting for that to change", self.crawl_date);
            
            let min_delay = Duration::from_secs(60);
            sleep(min_delay);

            return;
        }

        let db = Arc::new(Mutex::new(Vec::<FilingTransaction>::new()));

        // check for json file saved previously
        let path = self.get_file_path();
        let existing = Path::new(&path).exists();
        if existing {
            let file = File::open(&path);
            let rdr = BufReader::new(file.unwrap());

            let filings: Result<Vec<FilingTransaction>> = serde_json::from_reader(rdr);
            if filings.is_ok() {
                println!("Inserting from previously saved file {path}");
                Self::save_filings_db(&filings.unwrap())
                    .await
                    .expect("Error saving to db");
                
                self.increment_day();
                return;
            }
        }

        let body = get_daily_entries(self.crawl_date).await.unwrap();
        
        if body.len() == 0 {
            println!("Skip day {} index empty", self.crawl_date);
            self.increment_day();
            return;
        }

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

        Self::save_filings_db(&filings).await.expect("Should have saved to local file and db");
        
        self.increment_day();
    }
}