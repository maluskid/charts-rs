extern crate reqwest;
extern crate tokio;
mod stocks;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::{Error, ErrorKind};
use stocks::{display_stocks, get_stock_alpha, get_stock_rapid, StockJsonA, StockJsonR};

/// Branch is an enum which branches the main function depending upon
/// the arguments passed to the program. If an argument is followed by
/// more arguments, they will be contained within that enumeration.
enum Branch {
    Symbol(Vec<String>),
    Add(Vec<String>),
    Remove(Vec<String>),
    List,
    None,
}

const SNIP0: &str = "https://www.alphavantage.co/query?function=GLOBAL_QUOTE&symbol=";
const SNIP1: &str = "&apikey=1FGPYOV8MJGHJ1IC";
const USAGE: &str = "Usage:\n\tcharts-rs <symbol>\n\
                 \tcharts-rs add <symbol>\n\
                 \tcharts-rs rm <symbol>\n\
                 \tcharts-rs list\n\
                 \tcharts-rs list -s  <list name>\n\
                 \tcharts-rs list -n <list name>\n\
                 \tcharts-rs list -d <list name>\n\
                 \tcharts-rs list -a\n\n\
                 Enter charts-rs --help for more information.";
const HELP: &str =
    "Help:\n\tcharts-rs <symbol>\tEnter a list of symbols to display more than one.\n\
                 \tcharts-rs add <symbol>\tAdds a symbol or list of symbols to the watchlist.\n\
                 \tcharts-rs rm <symbol>\tRemoves a symbol or list of symbols from the watchlist.\n\
                 \tcharts-rs list\t\tLists the data for all symbols currently in the list.\n\
                 \tcharts-rs list -s <list name>\tSwitches the current list to <list name>\n\
                 \tcharts-rs list -n <list name>\tCreates a new empty list named <list name>\n\
                 \tcharts-rs list -d <list name>\tDeletes the list named <list name>\n\
                 \tcharts-rs list -a\t\tLists the names of all currently created lists\n";

#[tokio::main]
async fn main() {
    let mut args: Vec<String> = std::env::args().skip(1).collect();
    let stocks = match parse_args(&mut args) {
        Branch::Symbol(symbols) => retrieve_list(symbols).await,
        Branch::Add(symbols) => match append_list(symbols) {
            Ok(()) => None,
            Err(_) => {
                println!("Error appending symbols to list.\n");
                None
            }
        },
        Branch::Remove(symbols) => match edit_list(symbols) {
            Ok(()) => None,
            Err(e) => {
                println!("Error: {e}\n");
                None
            }
        },
        Branch::List => match read_list() {
            Some(list) => retrieve_list(list).await,
            None => {
                println!("List not found...\n");
                None
            }
        },
        Branch::None => None,
    };
    if stocks.is_some() {
        display_stocks(stocks.unwrap());
    }
}

/// Function that takes command line arguments and returns a Branch.
/// Branch items may contain a vector filled with stock tickers for
/// later use in the program, depending on given command.

fn parse_args(args: &mut Vec<String>) -> Branch {
    if args.len() == 0 {
        println!("{USAGE}");
        return Branch::None;
    }
    match &*args[0] {
        "--help" => {
            println!("{HELP}");
            Branch::None
        }
        "add" => {
            args.swap_remove(0);
            Branch::Add(args.clone())
        }
        "rm" => {
            args.swap_remove(0);
            Branch::Remove(args.clone())
        }
        "list" => {
            if args.len() > 3 {
                println!("Error: Invalid syntax.");
                return Branch::None;
            }
            // perhaps wrap an enum inisde of Branch::List?
            // is enumception too much? I don't want to put
            // functionality inside the parse_args function.
            match &*args[1] {
                "-s" => {
                    println!("TODO: list -s");
                    Branch::None
                }
                "-n" => {
                    println!("TODO: list -n");
                    Branch::None
                }
                "-d" => {
                    println!("TODO: list -d");
                    Branch::None
                }
                "-a" => {
                    println!("TODO: list -a");
                    Branch::None
                }
                _ => Branch::List,
            }
        }
        _ => Branch::Symbol(args.clone()),
    }
}

fn append_list(symbols: Vec<String>) -> Result<(), Error> {
    let mut list = OpenOptions::new()
        .append(true)
        .create(true)
        .open("list.txt")?;
    for symbol in symbols {
        list.write(symbol.as_bytes())?;
        list.write(b"\t")?;
    }
    Ok(())
}

fn edit_list(symbols: Vec<String>) -> Result<(), Error> {
    let new_list = match read_list() {
        Some(mut current_list) => {
            for del_item in symbols {
                let mut i: usize = 0;
                while i < current_list.len() {
                    if current_list[i] == del_item {
                        current_list.swap_remove(i);
                        i = current_list.len();
                    } else {
                        i += 1;
                    }
                }
            }
            Some(current_list)
        }
        None => None,
    };
    match new_list {
        None => Err(Error::new(ErrorKind::InvalidData, "File was empty!")),
        Some(l) => {
            let mut list = OpenOptions::new()
                .write(true)
                .truncate(true)
                .open("list.txt")?;
            for symbol in l {
                list.write(symbol.as_bytes())?;
                list.write(b"\t")?;
            }
            Ok(())
        }
    }
}

fn read_list() -> std::option::Option<Vec<String>> {
    let mut list = match OpenOptions::new().read(true).open("list.txt") {
        Ok(f) => f,
        Err(e) => {
            println!("Error {e} opening file.");
            return None;
        }
    };
    let mut s = String::new();
    let contents: Option<&str> = match list.read_to_string(&mut s) {
        Ok(0) => None,
        Ok(_) => Some(s.as_str()),
        Err(e) => panic!("Error {e} reading from file."),
    };
    match contents {
        Some(slice) => {
            let mut parsed_contents: Vec<String> = Vec::new();
            for item in slice.split('\t') {
                if item != "" {
                    parsed_contents.push(item.to_owned());
                }
            }
            Some(parsed_contents)
        }
        None => None,
    }
}

async fn retrieve_list(list: Vec<String>) -> std::option::Option<Vec<StockJsonA>> {
    let mut out: Vec<StockJsonA> = Vec::new();
    for symbol in list {
        println!("{symbol}");
        let url = format!("{}{}{}", SNIP0, symbol, SNIP1);
        match get_stock_alpha(url).await {
            Ok(s) => out.push(s),
            Err(e) => {
                println!("{e}");
            }
        }
    }
    Some(out)
}
