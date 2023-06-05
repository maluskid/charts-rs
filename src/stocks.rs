use reqwest;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;

const ALPHASNIP0: &str = "https://www.alphavantage.co/query?function=GLOBAL_QUOTE&symbol=";
const ALPHASNIP1: &str = "&apikey=1FGPYOV8MJGHJ1IC";
const MSSNIP0: &str = "";
const MSSNIP1: &str = "";

pub struct Stocks {
    symbols: Vec<String>,
    json: Vec<Json>,
}

// Serde structs to contain the Json information returned from various APIS
// Wrap these in a Json enum below
#[derive(Debug, Serialize, Deserialize)]
struct StockJsonAlphavantage {
    #[serde(rename = "Global Quote")]
    quote: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct StockJsonMsFinance {
    price: f32,
    change_point: f32,
    change_percentage: f32,
    total_vol: String,
}

// An enum which will contain different Json objects depending on API,
// for use within the Stocks struct
enum Json {
    MsFinance(Option<StockJsonMsFinance>),
    Alphavantage(Option<StockJsonAlphavantage>),
    None,
}

impl Stocks {
    pub fn from(items: Vec<String>) -> Stocks {
        Stocks {
            symbols: items,
            json: Vec::from([Json::None]),
        }
    }

    pub async fn display_stocks(&self) {
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

        // may not need this code, iterate over retrieve with Json type predetermined.

        let alpha_stocks = Vec::new();
        let msfinance_stocks = Vec::new();
        if self.symbols.len() > 5 {
            alpha_stocks = self.symbols.clone();
            msfinance_stocks = alpha_stocks.split_off(4);
        } else {
            alpha_stocks = self.symbols.clone();
        }

        // write a Json agnostic function to handle this portion
        for stock in self.json {
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
                Json::MsFinance(None) => match get_stock_ms(&symbol).await {
                    Ok(json) => Json::MsFinance(Some(json)),
                    Err(e) => {
                        println!("Error {e} fetching {symbol} from API.");
                        Json::None
                    }
                },
                _ => Json::None,
            }
        }
    }
}

/// Multiple implementations of get_stock to account for different
/// APIs. This uses the Alphavantage API, which sadly only allows
/// for 5 requests per minute. Its response time is quite fast though.
/// Plug these into the match statement of the retrieve() function.

async fn get_stock_alpha(symbol: &String) -> Result<StockJsonAlphavantage, Box<reqwest::Error>> {
    let url = format!("{}{}{}", ALPHASNIP0, symbol, ALPHASNIP1);
    let res = reqwest::Client::new().get(url).send().await?.text().await?;
    println!("Response: {}", res);
    let stock: StockJsonAlphavantage =
        serde_json::from_str(&res).unwrap_or(StockJsonAlphavantage::from(StockJsonAlphavantage {
            quote: HashMap::new(),
        }));
    Ok(stock)
}

/// Implementation for the 'MS Finance' API on Rapidapi. Main benefit of this API is
/// that it's free and has nearly unlimited requests.

async fn get_stock_ms(symbol: &String) -> Result<StockJsonMsFinance, Box<reqwest::Error>> {
    let url = format!("https://ms-finance.p.rapidapi.com/{}", symbol);
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
    let stock: StockJsonMsFinance = serde_json::from_str(&res).unwrap();
    // trying this with regular unwrap() first
    /* _or(StockJsonR::from(StockJsonR {
        price: 0.0,
        change_point: 0.0,
        change_percentage: 0.0,
        total_vol: "0".to_owned(),
    })); */
    Ok(stock)
}

// preserving old syntax of display_stocks function for reference or in case I need to go back
/* pub async fn display_stocks(stocks_string: Vec<String>) {
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
} */
