extern crate reqwest;
extern crate tokio;
mod stocks;
use std::io::{Error,ErrorKind};
use std::io::prelude::*;
use std::fs::OpenOptions;
use stocks::{display_stocks, get_stock, StockJson};

/// Branch is an enum which branches the main function depending upon
/// the arguments passed to the program. If an argument is followed by
/// more arguments, they will be contained within that enumeration.
enum Branch {
    Symbol(Vec<String>),
    Add(Vec<String>),
    Remove(Vec<String>),
    List,
    None
}


const SNIP0: &str = "https://www.alphavantage.co/query?function=GLOBAL_QUOTE&symbol=";
const SNIP1: &str = "&apikey=1FGPYOV8MJGHJ1IC";

#[tokio::main]
async fn main() {
    
    let mut args: Vec<String> = std::env::args().skip(1).collect();
    let stocks = match parse_args(&mut args) {
        Branch::Symbol(symbols) => {
            retrieve_list(symbols).await
        },
        Branch::Add(symbols) => {
            // Append symbols received as arguments to add command to the list
            match append_list(symbols) {
                Ok(()) => None,
                Err(_) => {
                    println!("Error appending symbols to list.\n");
                    None
                }
            }
        },
        Branch::Remove(symbols) => {
            // Attempt to remove symbols received as arguments to remove 
            // command from list file.
            match edit_list(symbols) {
                Ok(()) => None,
                Err(e) => {
                    println!("Error: {e}\n");
                    None
                }
            }
        }
        Branch::List => {
            match read_list() {
                Some(list) => retrieve_list(list).await,
                None => {
                    println!("List not found...\n");
                    None
                }
            }
        },
        Branch::None => {
            println!("Usage:\n\tcharts-rs <symbol>\n\tcharts-rs add <symbol>\n\tcharts-rs list\n\tcharts-rs rm <symbol>");
            None
        }
    };
    if stocks.is_some() { display_stocks(stocks.unwrap()); }
}

/// Function that takes command line arguments and returns a Branch.
/// Branch items may contain a vector filled with stock tickers for
/// later use in the program, depending on given command.

fn parse_args(args: &mut Vec<String>) -> Branch {
    if args.len() == 0 {
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
            Branch::Symbol(args.clone())
        } 
    }
}


fn append_list(symbols: Vec<String>) -> Result<(), Error> {
    let mut list = OpenOptions::new().append(true).create(true).open("list.txt")?;
    for symbol in symbols {
        list.write(symbol.as_bytes())?;
        list.write(b"\t")?;
    } 
    Ok(())
}

fn edit_list(symbols: Vec<String>) -> Result<(), Error> {
    let mut list = OpenOptions::new().write(true).open("list.txt")?;
    let mut s = String::new();
    let contents: Option<String> = match list.read_to_string(&mut s) {
        Ok(0) => None,
        Ok(_) => Some(s.as_str().to_owned()),
        Err(e) => panic!("Error {e} reading from file."),
    };
    if contents.is_none() {
        return Err(Error::new(ErrorKind::InvalidData, "File was empty!"));
    }
    for symbol in symbols {
    }
    Ok(())
}


fn read_list() -> std::option::Option<Vec<String>> {“Velocity is crucial in marketing. The more campaigns we can put together, the more pages we can create, the bigger we feel, and the more touch points we have with customers. Webflow unlocks that for us and allows us to do more with less.”
    let mut list = match OpenOptions::new().read(true).open("list.txt") {“Velocity is crucial in marketing. The more campaigns we can put together, the more pages we can create, the bigger we feel, and the more touch points we have with customers. Webflow unlocks that for us and allows us to do more with less.”
        Ok(f) => f,
        Err(e) => {
            println!("Error {e} opening file.");
            return None
        }
    };
    let mut s = String::new();
    let contents: Option<&str> = match list.read_to_string(&mut s) {
        Ok(0) => None,
        Ok(_) => Some(s.as_str()),
        Err(e) => panic!("Error {e} reading from file.")
    };
    match contents {
        Some(slice) => {
            let mut parsed_contents: Vec<String> = Vec::new();
            for item in slice.split('\t') {
                parsed_contents.push(item.to_owned());
            }
            Some(parsed_contents)
        },
        None => None
    }
}


async fn retrieve_list(list: Vec<String>) -> std::option::Option<Vec<StockJson>> {
    let mut out: Vec<StockJson> = Vec::new();
    for symbol in list {
        let url = format!("{}{}{}", SNIP0, symbol, SNIP1);
        match get_stock(url).await {
            Ok(s) => out.push(s),
            Err(e) => {
                if !e.is_decode() {
                    println!("{e}");
                    println!("Fetching stock from API failed");
                }
            }
        }
    }
    Some(out)
}
