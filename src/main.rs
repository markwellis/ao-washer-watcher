extern crate reqwest;
extern crate select;

use select::document::Document;
use select::predicate::Class;
use anyhow::{Result, anyhow};

const L8_URL: &str = "https://ao.com/product/l8wec166r-aeg-okomix-technology-washer-dryer-white-45659-2.aspx";
const L9_URL: &str = "https://ao.com/product/l9wec169r-aeg-sensidry-technology-washer-dryer-white-45660-2.aspx";

#[tokio::main]
async fn main() {
    //loop 
        match find_add_to_basket(L8_URL).await {
            Err(e) => eprintln!("{}", e),
            Ok(v) => if v {println!("IN STOCK {}", L8_URL)}
        }

        match find_add_to_basket(L9_URL).await {
            Err(e) => eprintln!("{}", e),
            Ok(v) => if v {println!("IN STOCK {}", L9_URL)}
        }
    //sleep
}

async fn find_add_to_basket(url: &str) -> Result<bool> {
    let resp = reqwest::get(L8_URL).await?;

    if resp.status().is_success() {
        return Err(anyhow!("request failed for {}", url));
    }

    let document = Document::from(resp.text().await?.as_str());

    let button = document.find(Class("addToBasket"));

    println!("c {}", button.count());

    // if !button.is_empty() {
    //     return Ok(true);
    // }

    return Ok(false)
}
