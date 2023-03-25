extern crate reqwest;
extern crate tokio;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[tokio::main]
async fn main() {
    let mut stocks: Vec<StockJson> = Vec::new();
    let api_key = "1FGPYOV8MJGHJ1IC";

    parse_args(std::env::args());

    // replacing this with parse_args
    let symbol = match std::env::args().skip(1).next() {
        Some(arg) => arg,
        None => panic!("No argument given")
    };
    // end code to be replaced
    
    let url = format!("https://www.alphavantage.co/query?function=GLOBAL_QUOTE&symbol={}&apikey={}", symbol, api_key);
    let stock = match get_stock(url).await {
        Ok(s) => s,
        Err(_) => panic!("Fetching stock from API failed")
    };
    stocks.push(stock);
    display_stocks(stocks);
}

fn parse_args(args: std::env::Args) {
    // return an enum with info about necessary follow up?
}

fn display_stocks(stocks: Vec<StockJson>) {
    println!("Hello from display_stocks");
}

#[derive(Debug, Serialize, Deserialize)]
struct StockJson {
    #[serde(rename = "Global Quote")]
    quote: HashMap<String, String>
}

async fn get_stock(url: String) -> Result<StockJson, Box<(dyn std::error::Error)>> {
    
    println!("Printing from get_stock: {}", url);
    let stock: StockJson = reqwest::Client::new()
        .get(url)
        .send()
        .await?
        .json()
        .await?;

    println!("Printing from get_stock:\n{:#?}\n", stock);
    Ok(stock)
}
