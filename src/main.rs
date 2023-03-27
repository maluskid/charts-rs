extern crate reqwest;
extern crate tokio;
mod stocks;
use stocks::{display_stocks, get_stock, StockJson};

enum Branch {
    Symbol(String),
    Add(Vec<String>),
    Remove(Vec<String>),
    List,
    None
}

#[tokio::main]
async fn main() {
    let api_key = "1FGPYOV8MJGHJ1IC";
    
    let mut args: Vec<String> = std::env::args().skip(1).collect();
    match parse_args(&mut args) {
        Branch::Symbol(symbol) => {
            let url = format!("https://www.alphavantage.co/query?function=GLOBAL_QUOTE&symbol={}&apikey={}", symbol, api_key);
            match get_stock(url).await {
                Ok(s) => Some(vec![s]),
                Err(_) => {
                    println!("Fetching stock from API failed");
                    None
                }
            }
        },
        Branch::Add(symbols) => {
            let mut out: Vec<StockJson> = Vec::new();
            for symbol in symbols {
                let url = format!("https://www.alphavantage.co/query?function=GLOBAL_QUOTE&symbol={}&apikey={}", symbol, api_key);
                match gkt_stock(url).await {
                    Ok(s) => out.push(s),
                    Err(_) => println!("Fetching stock from API failed")
                }
            }
            if out.len() > 0 { Some(out) }
            else { None }
        },
        Branch::Remove(symbols) => {
            println!("TODO:\nRemove these symbols {:?}", symbols);
            None
        }
        Branch::List => {
            println!("TODO\n");
            None
        },
        Branch::None => None
    };

}

/// Function that takes command line arguments and returns a Branch.
/// Branch items may contain a vector filled with stock tickers for
/// later use in the program, depending on given command.

fn parse_args(args: &mut Vec<String>) -> Branch {
    if args.len() == 0 {
        println!("Usage:\n\tcharts-rs <symbol>\n\tcharts-rs add <symbol>\n\tcharts-rs list\n\tcharts-rs rm <symbol>");
        return Branch::None;
    } 
    match &*args[0] {
        "add" => {
            args.swap_remove(0);
            Branch::Add(args.clone())
        },
        "rm" => {
            args.swap_remove(0);
            Branch::Remove(args.clone())
        },
        "list" => Branch::List,
        _ => {
            if args.len() > 1 { panic!("Invalid argument. Try charts-rs --help for more info.") }
            Branch::Symbol(args[0].clone())
        } 
    }
}


