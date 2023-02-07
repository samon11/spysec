use chrono::NaiveDate;
use chrono_tz::US::Eastern;
use spysec::crawler::Crawler;

#[tokio::main]
async fn main() {
    let start = chrono::Utc::now()
        .with_timezone(&Eastern)
        .date_naive();

    let end = NaiveDate::from_ymd_opt(2000, 1, 4).unwrap();
    let mut crawler = Crawler::new(&start);

    loop {
        if crawler.crawl_date == end {
            println!("Stop date reached {:?}", end);
            break;
        }

        crawler.run(8).await;
    }
}
