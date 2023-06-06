extern crate dirs;
extern crate reqwest;
extern crate tokio;
mod stocks;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::{Error, ErrorKind};
use std::net::{Shutdown, TcpStream};
use stocks::Stocks;

// Using const definitions for string literals to declutter code later on.
/* items removed from directions:
\tcharts-rs list -a\n\n\
\tcharts-rs list -a\t\tLists the names of all currently created lists\n"; */

const USAGE: &str = "Usage:\n\tcharts-rs <symbol>\n\
                 \tcharts-rs add <symbol>\n\
                 \tcharts-rs rm <symbol>\n\
                 \tcharts-rs list\n\
                 \tcharts-rs list -s <list name>\n\
                 \tcharts-rs list -n <list name>\n\
                 \tcharts-rs list -d <list name>\n\n\
                 Enter charts-rs --help for more information.";
const HELP: &str =
    "Help:\n\tcharts-rs <symbol>\t\tEnter a list of symbols to display more than one.\n\
                 \tcharts-rs add <symbol>\t\tAdds a symbol or list of symbols to the watchlist.\n\
                 \tcharts-rs rm <symbol>\t\tRemoves a symbol or list of symbols from the watchlist.\n\
                 \tcharts-rs list\t\t\tLists the data for all symbols currently in the list.\n\
                 \tcharts-rs list -s <list name>\tSwitches the current list to <list name>\n\
                 \tcharts-rs list -n <list name>\tCreates a new empty list named <list name> and sets it to the current list.\n\
                 \tcharts-rs list -d <list name>\tDeletes the list named <list name>\n";

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

#[tokio::main]
async fn main() {
    let current_list = get_current_list();
    let mut args: Vec<String> = std::env::args().skip(1).collect();
    match parse_args(&mut args) {
        // Branch::Symbol(symbols) => retrieve_list(symbols).await,
        Branch::Symbol(symbols) => {
            let mut data = Stocks::from(symbols);
            data.display_stocks().await;
        }
        Branch::Add(symbols) => match append_list(symbols, current_list) {
            Ok(()) => (),
            Err(_) => {
                println!("Error appending symbols to list.\n");
            }
        },
        Branch::Remove(symbols) => match edit_list(symbols, current_list) {
            Ok(()) => (),
            Err(e) => {
                println!("Error: {e}\n");
            }
        },
        Branch::List => match read_list(current_list) {
            // Some(list) => retrieve_list(list).await,
            Some(list) => {
                let mut data = Stocks::from(list);
                data.display_stocks().await;
            }
            None => {
                println!("List not found...\n");
            }
        },
        Branch::None => (),
    };
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
            match args.len() {
                1 => Branch::List,
                2..=3 => match &*args[1] {
                    "-s" => {
                        if args.len() != 3 {
                            println!("Error: Invalid syntax.");
                        } else {
                            set_current_list(args[2].clone());
                        }
                        Branch::None
                    }
                    "-n" => {
                        // Will need to change this if not using microservice
                        if args.len() != 3 {
                            println!("Error: Invalid syntax.");
                        } else {
                            set_current_list(args[2].clone());
                        }
                        Branch::None
                    }
                    "-d" => {
                        set_current_list("list".to_owned());
                        Branch::None
                    }
                    /* "-a" => {
                        println!("TODO: list -a");
                        Branch::None
                    }*/
                    _ => Branch::List,
                },
                4.. => {
                    println!("Error: Invalid syntax.");
                    return Branch::None;
                }
                _ => Branch::None,
            }
        }
        "exit" => {
            close_server().unwrap();
            Branch::None
        }
        _ => Branch::Symbol(args.clone()),
    }
}

fn close_server() -> Result<(), Error> {
    let mut client = TcpStream::connect("127.0.0.1:1080")?;
    client.write("EXIT".as_bytes())?;
    client.shutdown(Shutdown::Both)?;
    Ok(())
}

fn append_list(symbols: Vec<String>, current_name: String) -> Result<(), Error> {
    /* OLD CODE USING FILES ON NATIVE SYSTEM
    let mut list = OpenOptions::new()
        .append(true)
        .create(true)
        .open("list.txt")?;
    for symbol in symbols {
        list.write(symbol.as_bytes())?;
        list.write(b"\t")?;
    } */
    let mut client = TcpStream::connect("127.0.0.1:1080")?;
    let mut message = format!("POST {current_name} ");
    for symbol in symbols {
        let tmp = format!("{symbol} ");
        message.push_str(tmp.as_str());
    }
    client.write(message.as_bytes())?;
    client.shutdown(Shutdown::Both)?;
    Ok(())
}

fn edit_list(symbols: Vec<String>, current_name: String) -> Result<(), Error> {
    let new_list = match read_list(current_name) {
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

fn read_list(current_name: String) -> std::option::Option<Vec<String>> {
    let mut client = TcpStream::connect("127.0.0.1:1080").unwrap();
    let message = format!("GET {current_name}");
    client.write(message.as_bytes()).unwrap();
    let mut s = String::new();
    let contents: Option<&str> = match client.read_to_string(&mut s) {
        Ok(0) => None,
        Ok(_) => Some(s.as_str()),
        Err(e) => panic!("Error {e} reading from file."),
    };
    match contents {
        Some(slice) => {
            let mut parsed_contents: Vec<String> = Vec::new();
            for item in slice.split(' ') {
                if item != "" {
                    parsed_contents.push(item.to_owned());
                }
            }
            Some(parsed_contents)
        }
        None => None,
    }
    /* Old code using local files
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
    } */
}

/* async fn retrieve_list(list: Vec<String>) -> std::option::Option<Vec<StockJsonA>> {
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
} */

fn set_current_list(mut new_name: String) -> () {
    let path: std::path::PathBuf = match dirs::config_dir() {
        Some(mut out) => {
            out.push(".charts-rs/current_list.txt");
            out
        }
        None => {
            if let Some(mut out) = dirs::home_dir() {
                out.push(".charts-rs/current_list.txt");
                out
            } else {
                panic!("Could not find .charts-rs directory!");
            }
        }
    };
    new_name.push_str(".txt");
    match OpenOptions::new().write(true).truncate(true).open(&path) {
        Ok(mut f) => {
            f.write(new_name.as_bytes()).unwrap();
        }
        Err(e) => {
            panic!("Error {e} opening file.");
        }
    }
}

fn get_current_list() -> String {
    let path: std::path::PathBuf = match dirs::config_dir() {
        Some(mut out) => {
            out.push(".charts-rs/current_list.txt");
            out
        }
        None => {
            if let Some(mut out) = dirs::home_dir() {
                out.push(".charts-rs/current_list.txt");
                out
            } else {
                panic!("Could not find .charts-rs directory!");
            }
        }
    };
    match OpenOptions::new().read(true).open(&path) {
        Ok(mut f) => {
            let mut buf = String::new();
            f.read_to_string(&mut buf).unwrap();
            buf
        }
        Err(e) => {
            if e.kind() == ErrorKind::NotFound {
                let mut temp = OpenOptions::new()
                    .read(true)
                    .write(true)
                    .create(true)
                    .open(&path)
                    .unwrap();
                temp.write("list.txt".as_bytes()).unwrap();
                let out = String::from("list.txt");
                out
            } else {
                panic!("Error {e} opening file.");
            }
        }
    }
}
