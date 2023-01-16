use chrono::NaiveDate;
use spysec::crawler::Crawler;

#[tokio::main]
async fn main() {
    let start = NaiveDate::from_ymd_opt(2023, 1, 4).unwrap();
    let crawler = Crawler::new(&start);

    crawler.run(10).await;
}
