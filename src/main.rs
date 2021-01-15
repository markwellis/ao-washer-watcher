extern crate reqwest;
extern crate select;

use select::document::Document;
use select::predicate::Class;

const L8_URL: &str = "https://ao.com/product/l8wec166r-aeg-okomix-technology-washer-dryer-white-45659-2.aspx";
const L9_URL: &str = "https://ao.com/product/l9wec169r-aeg-sensidry-technology-washer-dryer-white-45660-2.aspx";

#[tokio::main]
async fn main() {
    find_add_to_basket(L8_URL).await;
    find_add_to_basket(L9_URL).await;
}

async fn find_add_to_basket(url: &str) {
    let res = reqwest::get(url).await.unwrap();

    if !res.status().is_success() {
        panic!("request failed for {}", url);
    }
    let path = res.url().path().to_string();

    let document = Document::from(res.text().await.unwrap().as_str());

    let button = document.find(Class("addToBasket"));

    if button.count() > 0 {
        println!("IN STOCK {}", path);
    }
}
