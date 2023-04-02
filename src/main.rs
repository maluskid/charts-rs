extern crate reqwest;
extern crate tokio;
mod stocks;
use std::io::prelude::*;
use std::fs::{ File, OpenOptions };
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

    let snip0 = "https://www.alphavantage.co/query?function=GLOBAL_QUOTE&symbol=";
    let snip1 = "&apikey=1FGPYOV8MJGHJ1IC";
    
    let mut args: Vec<String> = std::env::args().skip(1).collect();
    let stocks = match parse_args(&mut args) {
        Branch::Symbol(symbol) => {
            let url = format!("{}{}{}", snip0, symbol, snip1);
            match get_stock(url).await {
                Ok(s) => Some(vec![s]),
                Err(_) => {
                    println!("Fetching stock from API failed");
                    None
                }
            }
        },
        Branch::Add(symbols) => {
            append_list(symbols);
            let mut out: Vec<StockJson> = Vec::new();
            for symbol in symbols {
                let url = format!("{}{}{}", snip0, symbol, snip1);
                match get_stock(url).await {
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
            let symbols = read_list();
            let mut out: Vec<StockJson> = Vec::new();
            for symbol in symbols {
                let url = format!("{}{}{}", snip0, symbol, snip1);
                match get_stock(url).await {
                    Ok(s) => out.push(s),
                    Err(_) => println!("Fetching stock from API failed")
                }
            }
            if out.len() > 0 { Some(out) }
            else { None }
        },
        Branch::None => None
    };
    if stocks.is_some() { display_stocks(stocks.unwrap()); }
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

/// Function to open the stored watchlist or create one if none exists
/// returns the opened file for further editing
fn open_list() -> File {
    let out = match OpenOptions::new()
        .read(true)
        .append(true)
        .create(true)
        .open("list.txt") {
            Ok(f) => f,
            Err(e) => panic!("Error {e} opening file.")
        };
    out
}

fn append_list(symbols: Vec<String>) -> Result<(), std::io::Error> {
    let mut list = open_list();
    for symbol in symbols {
        list.write(symbol.as_bytes())?;
        list.write(b"\t")?;
    } 
    Ok(())
}


// Possibly rewrite append_list and read_list to OpenOptions with their own relevant
// options and delete open_list
fn read_list() -> Vec<String> {
    let mut list = open_list();
    let out: Vec<String> = Vec::new();
    let mut s = String::new();
    let contents = match list.read_to_string(&mut s) {
        Ok(num) => s.as_str(),
        Err(e) => panic!("Error {e} reading from file.")
    };
    loop {
        let index = match contents.find('\t') {
            Some(num) => num,
            None => 0
        };
    }
    
}

