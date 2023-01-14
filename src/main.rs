use spysec::secweb::get_form;
use std::time::{SystemTime};


fn main() {
    let now = SystemTime::now();
    let url = "https://www.sec.gov/Archives/edgar/data/1014894/0001085146-23-000124.txt";
    let body = get_form(url).expect("Request failed to get root page");
    println!("Response: {:?}", body);

    match now.elapsed() {
        Ok(elapsed) => {
            println!("Latency (ms): {}", elapsed.as_millis())
        },
        Err(ex) => {
            println!("Error {ex:?}");
        }
    }

}
