extern crate reqwest;
extern crate tokio;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[tokio::main]
async fn main() {
    let api_key = "1FGPYOV8MJGHJ1IC";
    let symbol =  match std::env::args().skip(1).next() {
        Some(arg) => arg,
        None => panic!("No argument given")
    }; 
    let url = format!("https://www.alphavantage.co/query?function=GLOBAL_QUOTE&symbol={}&apikey={}", symbol, api_key);
    let stock = get_stock(url)
        .await;

    println!("Printing from main:\n{:#?}\n", stock);
}

#[derive(Debug, Serialize, Deserialize)]
struct Stock {
    #[serde(rename = "Global Quote")]
    quote: HashMap<String, String>
}

async fn get_stock(url: String) -> Result<Stock, Box<(dyn std::error::Error)>> {
    
    println!("Printing from get_stock: {}", url);
    let stock: Stock = reqwest::Client::new()
        .get(url)
        .send()
        .await?
        .json()
        .await?;

    println!("Printing from get_stock:\n{:#?}\n", stock);
    Ok(stock)
}
