use reqwest;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

pub fn display_stocks(stocks: Vec<StockJson>) {
    println!("Hello from display_stocks");
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StockJson {
    #[serde(rename = "Global Quote")]
    quote: HashMap<String, String>
}

pub async fn get_stock(url: String) -> Result<StockJson, Box<(dyn std::error::Error)>> {
    
    let stock: StockJson = reqwest::Client::new()
        .get(url)
        .send()
        .await?
        .json()
        .await?;

    Ok(stock)
}
