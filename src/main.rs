use chrono::NaiveDate;
use spysec::crawler::Crawler;

#[tokio::main]
async fn main() {
    let start = NaiveDate::from_ymd_opt(2020, 1, 4).unwrap();
    let end = NaiveDate::from_ymd_opt(2022, 1, 3).unwrap();
    let mut crawler = Crawler::new(&start);

    loop {
        if crawler.crawl_date == end {
            println!("Stop date reached {:?}", end);
            break;
        }

        crawler.run(9).await;
    }
}
