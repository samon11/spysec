use chrono::NaiveDate;
use spysec::secweb::models::FilingTransaction;
use spysec::secweb::{get_daily_entries, process_entries};
use spysec::secweb::persist::save_filings;
use std::{time::{Duration}, sync::{Arc, Mutex}, thread::sleep};

#[tokio::main]
async fn main() {
    let db = Arc::new(Mutex::new(Vec::<FilingTransaction>::new()));

    let day = NaiveDate::from_ymd_opt(2022, 1, 10).unwrap();
    let body = get_daily_entries(day).await.unwrap();
    
    assert_ne!(body.len(), 0);
    let second_delay = Duration::from_secs(1);
    
    let mut skip = 0;
    let batch = 5;
    let total = body.len() / batch;
    for i in 0..total {
        println!("Get {i}/{total}");
        let _res = process_entries(&body, db.clone(), skip, batch).await;
        skip += batch;
        sleep(second_delay);
    }

    sleep(second_delay);

    let filings = db.lock().unwrap();
    println!("Filing items: {:?}", filings.len());
    save_filings("filings.json", &filings).unwrap();
}
