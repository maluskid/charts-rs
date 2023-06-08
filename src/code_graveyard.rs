/* items removed from directions:
\tcharts-rs list -a\n\n\
\tcharts-rs list -a\t\tLists the names of all currently created lists\n"; */

//file implementation taken from main.rs
/* from append_list OLD CODE USING FILES ON NATIVE SYSTEM
let mut list = OpenOptions::new()
    .append(true)
    .create(true)
    .open("list.txt")?;
for symbol in symbols {
    list.write(symbol.as_bytes())?;
    list.write(b"\t")?;
} */
/* Old code using local files from
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

// code taken from stocks.rs
const MSSNIP0: &str = "https://ms-finance.p.rapidapi.com/market/v2/auto-complete?q=";
const MSSNIP1: &str =
    "https://ms-finance.p.rapidapi.com/market/v2/get-realtime-data?performanceIds=";
const RAPIDKEY: &str = "216c8810b8msh81fd3895966c048p1f50b6jsn9dbb47c8f68e";

fn format_ms(stocks: Vec<Value>) -> String {
    let out = String::new();

    out
}

/// Implementation for the 'MS Finance' API on Rapidapi. Main benefit of this API is
/// that it's free and has nearly unlimited requests.

async fn get_id_ms(symbol: &String) -> Result<String, Box<reqwest::Error>> {
    let url = format!("{MSSNIP0}{symbol}");
    let res = reqwest::Client::new()
        .request(reqwest::Method::GET, url)
        .header("X-RapidAPI-Key", RAPIDKEY)
        .header("X-RapidAPI-Host", "ms-finance.p.rapidapi.com")
        .send()
        .await?
        .text()
        .await?;
    let ms_response: Value = serde_json::from_str(&res).unwrap();
    let mut ms_id = ms_response["results"][0]["performanceId"].to_string();
    ms_id.remove(0);
    ms_id.remove(ms_id.len() - 1);
    Ok(ms_id)
}

async fn get_stock_ms(ids: Vec<String>) -> Result<Vec<Value>, Box<reqwest::Error>> {
    let mut out: Vec<Value> = Vec::new();
    let mut id_string = String::new();
    for id in &ids {
        println!("id adding to string: {id}");
        id_string.push_str(format!("{id}%2C").as_str());
    }
    id_string.drain(id_string.len() - 4..id_string.len() - 1);
    println!("id_string: {id_string}");
    let url = format!("{MSSNIP1}{id_string}");
    println!("url: {url}");
    let res = reqwest::Client::new()
        .request(reqwest::Method::GET, url)
        .header("X-RapidAPI-Key", RAPIDKEY)
        .header("X-RapidAPI-Host", "ms-finance.p.rapidapi.com")
        .send()
        .await?
        .text()
        .await?;
    let ms_response: Value = serde_json::from_str(&res).unwrap();
    for id in &ids {
        println!("Adding {:?} to list.\n", &ms_response[id]);
        out.push(ms_response[id].clone())
    }

    Ok(out)
}
