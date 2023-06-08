use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::{self};
use std::collections::HashMap;

const ALPHASNIP0: &str = "https://www.alphavantage.co/query?function=GLOBAL_QUOTE&symbol=";
const ALPHASNIP1: &str = "&apikey=2FY0A0YAKP0OF93E";
// const ALPHASNIP1: &str = "&apikey=1FGPYOV8MJGHJ1IC";

pub struct Stocks {
    symbols: Vec<String>,
    json: Vec<Json>,
}

// Serde structs to contain the Json information returned from various APIS
// Wrap these in a Json enum below
#[derive(Serialize, Deserialize)]
struct StockJsonAlphavantage {
    #[serde(rename = "Global Quote")]
    quote: HashMap<String, String>,
}

// An enum which will contain different Json objects depending on API,
// for use within the Stocks struct
enum Json {
    Alphavantage(Option<StockJsonAlphavantage>),
    None,
}

impl Stocks {
    pub fn from(items: Vec<String>) -> Stocks {
        Stocks {
            symbols: items,
            json: Vec::new(),
        }
    }

    pub async fn display_stocks(&mut self) {
        let mut first = true;
        for symbol in &self.symbols {
            if first {
                print!("Loading[");
            } else {
                print!("===");
            }
            let data = retrieve(symbol.clone(), Json::Alphavantage(None)).await;
            self.json.push(data);
            first = false;
        }
        print!("]\n\n\n");

        Stocks::print_stocks(self).await;
    }

    async fn print_stocks(&self) {
        let dash = '-';
        let headers = [" Symbol ", " Price  ", " Prev   ", " Change ", " Pct %  "];

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

        let mut s = String::new();
        // Add your own API here
        for stock in &self.json {
            match stock {
                Json::Alphavantage(Some(data)) => s.push_str(format_alpha(data, &headers).as_str()),
                _ => println!("\t---------------------------------------------------"),
            };
        }
        println!("{s}");
    }
}

// Function to add to for new api implementation
async fn retrieve(symbol: String, api_kind: Json) -> Json {
    // insert your own api resolving function here
    match api_kind {
        Json::Alphavantage(None) => match get_stock_alpha(&symbol).await {
            Ok(json) => Json::Alphavantage(Some(json)),
            Err(e) => {
                println!("Error {e} fetching {symbol} from API.");
                Json::None
            }
        },
        _ => Json::None,
    }
}

fn format_alpha(stock: &StockJsonAlphavantage, headers: &[&str; 5]) -> String {
    let header_map = HashMap::from([
        (headers[0], "01. symbol"),
        (headers[1], "05. price"),
        (headers[2], "08. previous close"),
        (headers[3], "09. change"),
        (headers[4], "10. change percent"),
    ]);

    let mut s = String::from("\t");
    let default = String::from("N/A");

    for i in 0..headers.len() {
        s.push(' ');
        let key = header_map.get(&headers[i]).unwrap().to_string();
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
    s.push('\n');
    s
}

/// Multiple implementations of get_stock to account for different
/// APIs. This uses the Alphavantage API, which sadly only allows
/// for 5 requests per minute. Its response time is quite fast though.
/// Plug these into the match statement of the retrieve() function.

async fn get_stock_alpha(symbol: &String) -> Result<StockJsonAlphavantage, Box<reqwest::Error>> {
    let url = format!("{}{}{}", ALPHASNIP0, symbol, ALPHASNIP1);
    let res = reqwest::Client::new().get(url).send().await?.text().await?;
    let stock: StockJsonAlphavantage =
        serde_json::from_str(&res).unwrap_or(StockJsonAlphavantage::from(StockJsonAlphavantage {
            quote: HashMap::new(),
        }));
    Ok(stock)
}
