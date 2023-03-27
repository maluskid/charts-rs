use reqwest;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

pub fn display_stocks(stocks: Vec<StockJson>) {

    let dash = '\u{23AF}';
    let mut h_line = String::new();
    h_line.push('\n');
    for _ in 0..45 {
        h_line.push(dash);
    }
    h_line.push('\n');

    let headers = [
        " Symbol ",
        " Price  ",  
        " Prev   ",
        " Change ",
        " Pct %  "
    ];

    let header_map = HashMap::from([
        (headers[0],  "01. symbol"),
        (headers[1],  "05. price"),
        (headers[2],  "08. previous close"),
        (headers[3],  "09. change"),
        (headers[4],  "10. change percent")
    ]);

    print!("{h_line}");
    print!("|{}|{}|{}|{}|{}|",
        headers[0],
        headers[1],
        headers[2],
        headers[3],
        headers[4]
    );
    print!("{h_line}");

    for stock in stocks {
        let mut s = String::from('|');
        let default = String::from("N/A");
        for i in 0..5 {
            s.push(' ');
            let key = header_map
                .get(headers[i])
                .unwrap()
                .to_string();
            let mut value = stock.quote
                .get(&key)
                .unwrap_or(&default)
                .to_owned();
            if i > 0 && i < 4 {
                for _ in 0..2 { value.pop(); }
            }
            if i == 4 {
                for _ in 0..3 { value.pop(); }
                value.push('%');
            }
            s.push_str(&value);
            if value.len() < 7 {
                for _ in 0..(7 - value.len()) { s.push(' '); }
            }
            s.push('|');
        }
        print!("{s}{h_line}");
    }
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
