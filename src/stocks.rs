use reqwest;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;

pub fn display_stocks(stocks: Vec<StockJsonA>) {
    let dash = '-';
    let headers = [" Symbol ", " Price  ", " Prev   ", " Change ", " Pct %  "];

    let header_map = HashMap::from([
        (headers[0], "01. symbol"),
        (headers[1], "05. price"),
        (headers[2], "08. previous close"),
        (headers[3], "09. change"),
        (headers[4], "10. change percent"),
    ]);

    print!(
        "\t{}\t{}\t{}\t{}\t{}\n",
        headers[0], headers[1], headers[2], headers[3], headers[4]
    );

    for item in headers {
        print!("\t");
        for _ in 0..item.len() {
            print!("{dash}");
        }
    }
    print!("\n");

    for stock in stocks {
        let mut s = String::from("\t");
        let default = String::from("N/A");
        for i in 0..headers.len() {
            s.push(' ');
            let key = header_map.get(headers[i]).unwrap().to_string();
            let mut value = stock.quote.get(&key).unwrap_or(&default).to_owned();
            if i > 0 && i < 4 {
                for _ in 0..2 {
                    value.pop();
                }
            }
            if i == 4 {
                for _ in 0..3 {
                    value.pop();
                }
                value.push('%');
            }
            s.push_str(&value);
            if value.len() < 7 {
                for _ in 0..(7 - value.len()) {
                    s.push(' ');
                }
            }
            s.push('\t');
        }
        print!("{s}\n");
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StockJsonA {
    #[serde(rename = "Global Quote")]
    quote: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StockJsonR {
    price: f32,
    change_point: f32,
    change_percentage: f32,
    total_vol: String,
}

/// Multiple implementations of get_stock to account for different
/// APIs. This uses the Alphavantage API, which sadly only allows
/// for 5 requests per minute. Its response time is quite fast though.

pub async fn get_stock_alpha(url: String) -> Result<StockJsonA, Box<reqwest::Error>> {
    let res = reqwest::Client::new().get(url).send().await?.text().await?;
    println!("Response: {}", res);
    let stock: StockJsonA = serde_json::from_str(&res).unwrap_or(StockJsonA::from(StockJsonA {
        quote: HashMap::new(),
    }));
    Ok(stock)
}

/// Implementation for the 'realstonks' API on Rapidapi. Main benefit of this API is
/// that it's free and has nearly unlimited requests. Response time is a
/// little slower, however.

pub async fn get_stock_rapid(symbol: String) -> Result<StockJsonR, Box<reqwest::Error>> {
    let url = format!("https://realstonks.p.rapidapi.com/{}", symbol);
    let res = reqwest::Client::new()
        .request(reqwest::Method::GET, url)
        .header(reqwest::header::CONTENT_TYPE, "application/octet-stream")
        .header(
            "X-RapidAPI-Key",
            "216c8810b8msh81fd3895966c048p1f50b6jsn9dbb47c8f68e",
        )
        .header("X-RapidAPI-Host", "realstonks.p.rapidapi.com")
        .send()
        .await?
        .text()
        .await?;
    println!("Response: {}", res);
    let stock: StockJsonR = serde_json::from_str(&res).unwrap_or(StockJsonR::from(StockJsonR {
        price: 0.0,
        change_point: 0.0,
        change_percentage: 0.0,
        total_vol: "0".to_owned(),
    }));
    Ok(stock)
}
