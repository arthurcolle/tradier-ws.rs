#![allow(dead_code, unused_variables, warnings, unused)]

use std::collections::BTreeMap;
use std::collections::HashMap;
use polars::export::num::Float;
use reqwest::header::{AUTHORIZATION, CONTENT_LENGTH, ACCEPT};
use dotenv::dotenv;
use tokio::io::Stdout;
use std::fmt;
use std::io::{self, stdout};
use std::io::prelude::*;
use std::env::var;
use url::Url;
use tungstenite::{connect, Message};
use substring::Substring;
use websocket::ClientBuilder;
use ::chrono::{DateTime, Utc};
use serde_json::{Value, json};
use serde::{Deserialize, Serialize, Deserializer, de};
use serde_with::{serde_as, DisplayFromStr};
use std::io::stdin;
use std::sync::mpsc::channel;
use std::thread;
use std::fs;
use std::io::Write;
use std::result::Result;
use ftp::FtpStream;
use std::io::Cursor;

#[derive(Deserialize, Serialize, Debug)]
pub struct TimeSale {
  symbol: String,
  #[serde(rename = "type")]
  type_: String,
  exch: String,
  #[serde(deserialize_with = "de_s2f64")]
  bid: f64,
  #[serde(deserialize_with = "de_s2f64")]
  ask: f64,
  #[serde(deserialize_with = "de_s2f64")]
  last: f64,
  #[serde(deserialize_with = "de_s2f64")]
  size: f64,
  date: String  
}

fn de_s2f64<'de, D: Deserializer<'de>>(deserializer: D) -> Result<f64, D::Error> {
    Ok(match Value::deserialize(deserializer)? {
        Value::String(s) => s.parse().map_err(de::Error::custom)?,
        Value::Number(num) => num.as_f64().ok_or(de::Error::custom("Invalid number"))? as f64,
        _ => return Err(de::Error::custom("wrong type"))
    })
}

pub struct TimeSaleData {
  pub symbol: String,
  pub exch: String,
  pub bid: f64,
  pub ask: f64,
  pub last: f64,
  pub size: i32,
  pub date: String
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Trade {
  #[serde(rename = "type")]
  type_: String,
  symbol: String,
  exch: String,
  #[serde(deserialize_with = "de_s2f64")]
  price: f64,
  #[serde(deserialize_with = "de_s2f64")]
  size: f64,
  #[serde(deserialize_with = "de_s2f64")]
  cvol: f64,
  date: String,
  #[serde(deserialize_with = "de_s2f64")]
  last: f64
}

#[derive(Deserialize, Debug)]
pub struct Quote {
    #[serde(rename = "type")]
    type_: String,
    symbol: String,
    bid: f64,
    bidsz: i32,
    biddate: String,
    ask: f64,
    asksz: i32,
    askexch: String,
    askdate: String
}


#[derive(Deserialize, Serialize, Debug)]
pub struct Summary {
    #[serde(rename = "type")]
    type_: String,
    symbol: String,
    #[serde(deserialize_with = "de_s2f64")]
    open: f64,
    #[serde(deserialize_with = "de_s2f64")]
    high: f64,
    #[serde(deserialize_with = "de_s2f64")]
    low: f64,
    // #[serde(deserialize_with = "de_s2f64")]
    close: Option<String>,
    // #[serde(deserialize_with = "de_s2f64")]
    prevClose: Option<String>
}

pub struct SummaryData {
    pub symbol: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub prevClose: f64
}

pub struct TradeData {
    pub symbol: String,
    pub price: f64,
    pub size: i32,
    pub cvol: i32,
    pub date: String,
    pub last: f64
}

  

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum GenericResponseResult {    
    Trade(Trade),
    Quote(Quote),
    TimeSale(TimeSale),
    Summary(Summary)
}

#[derive( Debug, Deserialize, Serialize)]
pub struct MarketPayload {
    symbols: Vec<String>,
    sessionid: String,
    linebreak: bool
}

pub enum OrderType {
  BuyToOpen,
  SellToClose,
  BuyToClose,
  SellToOpen,
}

pub enum OrderAttribute {
  LimitSell,
  LimitBuy,
  MarketSell,
  MarketBuy,
  Vwap,
  Twap,
  Custom
}

pub struct Venue {
  code: String,
  name: String,
  description: String
}

pub struct Ticker {
  symbol: String,
  venue: Venue
}

pub struct Price {
  quantity: f64,
  currency: String
}

// pub struct Option {
//   ticker: &Ticker,
//   strike_price: &Price,
//   expiration_date: &DateTime,
//   current_price: Price,
//   last_trade_trade: Price,
//   delta: Greek,
//   gamma: Greek,
//   theta: Greek,
//   rho: Greek,
//   vega: Greek,
//   iv: ImpliedVolatility
// }

use crate::GenericResponseResult::*;

pub fn create_payload(symbols: Vec<String>, sessionid: String, linebreak: bool) -> String {
    let payload = MarketPayload {
        symbols: symbols.clone(),
        sessionid: sessionid.clone(),
        linebreak: linebreak.clone()
    };
    return json!(payload).to_string();
}
pub async fn process(grr: GenericResponseResult) -> bool {
    match grr {
        Summary(summary) => {
            println!(" [SUMMARY] {:#?}", summary);
        },
        Quote(quote) => {
            println!("{} [QUOTE] Nmid: ${:.2}", quote.symbol, (quote.bid+quote.ask)/2.0);
            let bid = quote.bid as f64;
            let offer = quote.ask as f64;
            let bidsz = quote.bidsz as f64;
            let offersz = quote.asksz as f64;
            println!("{} [QUOTE] Wmid ${:.2}", quote.symbol, (quote.bid*bidsz+offer*offersz)/(bidsz+offersz));
        },
        TimeSale(timesale) => {
            println!("{} [TIMESALE] {}", timesale.symbol, timesale.last);
        },
        Trade(trade) => {
            //    [TRADE] Trade {
            //     type_: "trade",
            //     symbol: "TSLA221223C00125000",
            //     exch: "N",
            //     price: "3.2",
            //     size: "3",
            //     cvol: "50689",
            //     date: "1671735204536",
            //     last: "3.2",
            // }
            
            println!("{} [TRADE] {}", trade.symbol, trade.price);
        }
    }
    return true;
}

pub async fn subscribe(payload: String) -> ! {
    let endpoint_url = "wss://ws.tradier.com/v1/markets/events";
    let (mut socket, response) =
        connect(Url::parse(endpoint_url).unwrap()).expect("Can't connect");

    println!("Connected to the server");
    println!("Response HTTP code: {}", response.status());
    println!("Response contains the following headers:");

    match socket.write_message(Message::Text(payload)) {
        Ok(T) => { println!("all good") }
        Err(_) => todo!(),
    }
    
    loop {
        let msg = socket.read_message().expect("Error reading message");
        // println!("{}", &msg);
        let event = generic_parse(msg.to_string()).await;
        let e = match event {
            GenericResponseResult::Summary(summary) => {
                println!(" [SUMMARY] {:#?}", summary);
                process(GenericResponseResult::Summary(summary)).await
            },
            GenericResponseResult::Quote(quote) => {
                println!("{} [QUOTE] Nmid: ${:.2}", quote.symbol, (quote.bid+quote.ask)/2.0);
                let bid = quote.bid as f64;
                let offer = quote.ask as f64;
                let bidsz = quote.bidsz as f64;
                let offersz = quote.asksz as f64;
                println!("{} [QUOTE] Wmid ${:.2}", quote.symbol, (quote.bid*bidsz+offer*offersz)/(bidsz+offersz));
                process(GenericResponseResult::Quote(quote)).await
            },
            GenericResponseResult::Trade(trade) => {
                println!("   [TRADE] {:#?}", trade);
                process(GenericResponseResult::Trade(trade)).await
            },
            GenericResponseResult::TimeSale(ts) => {
                println!("[TIMESALE] {:#?}", ts);
                process(GenericResponseResult::TimeSale(ts)).await
            }
        };
        // let e: TradierEvent = serde_json::from_str(&msg.to_string()).unwrap();
        // println!("Parsed event: {:?}", e);
    }
}

impl fmt::Display for SessionDataL1 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(stream: {:?})", self.stream)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SessionDataL1 {
    #[serde(flatten)]
    stream: HashMap<String, SessionDataL2>
}

impl fmt::Display for SessionDataL2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(url: {}, sessionid: {}", self.url, self.sessionid)
    }
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct SessionDataL2 {
    url: String,
    sessionid: String
}

#[warn(unused_variables)]
pub async fn market_session(access_token: &str) -> String {
    let client = reqwest::Client::new();
    let auth_header: String = format!("Bearer {access_token}");
    let data = client
        .post("https://api.tradier.com/v1/markets/events/session")
        .header(AUTHORIZATION, &auth_header)
        .header(CONTENT_LENGTH, 0)
        .header(ACCEPT, "application/json")
        .send()
        .await
        .expect("Failed to get response")
        .text()
        .await
        .expect("Failed to get payload");

    let d: SessionDataL1 = serde_json::from_str(&data).unwrap();
    let d1 = json!(d);

    let url = format!("{}", &d1["stream"]["url"]);
    let sessionid = format!("{}", &d1["stream"]["sessionid"]);
    let sid: String = format!("{}", sessionid.substring(1, sessionid.len()-1));
    return sid;
}

#[warn(unused_variables)]
pub async fn account_session(access_token: &str) -> String {
    let client = reqwest::Client::new();
    let auth_header: String = format!("Bearer {access_token}");
    let data = client
        .post("https://api.tradier.com/v1/accounts/events/session")
        .header(AUTHORIZATION, &auth_header)
        .header(CONTENT_LENGTH, 0)
        .header(ACCEPT, "application/json")
        .send()
        .await
        .expect("Failed to get response")
        .text()
        .await
        .expect("Failed to get payload");

    let d: SessionDataL1 = serde_json::from_str(&data).unwrap();
    let d1 = json!(d);

    let url = format!("{}", &d1["stream"]["url"]);
    let sessionid = format!("{}", &d1["stream"]["sessionid"]);
    let sid: String = format!("{}", sessionid.substring(1, sessionid.len()-1));
    return sid;
}

pub async fn interactive() {
    dotenv().ok();

    let tradier_access_token = 
        std::env::var("TRADIER_API_KEY")
            .expect("Access token not available!");

    let session_id = market_session(&tradier_access_token).await;

    print!(r#"Enter tickers (space sep.): "#);
    match std::io::stdout().flush() {
        Ok(_) => { print!("") },
        Err(error) => println!("{}", error),
    }

    let mut tickers: String = String::from("");

    io::stdin()
        .read_line(&mut tickers)
        .expect("Failed getting ticker input!");

    let tkrs = tickers.split(" ").collect::<Vec<_>>();
    let symbols: Vec<_> = tkrs.iter().map(|s| s.to_string()).collect();
    let payload = create_payload(symbols, session_id, true);

    subscribe(payload).await;
}

async fn generic_parse(variant: String) -> GenericResponseResult {
  let event: GenericResponseResult = serde_json::from_str(&variant).unwrap();
  return event;
}

pub async fn sandbox() {
    println!("Yo!")
}

fn option_price_lattice(underlier_price: f64, strike_price: f64, volatility: f64, interest_rate: f64, dividend_yield: f64, expiration: f64, steps: u32, option_type: OptionType) -> f64 {
    // Calculate the time step
    let dt = expiration / steps as f64;

    // Calculate the up and down factors
    let u = (1.0 + volatility * dt.sqrt()).powf(2.0);
    let d = (1.0 - volatility * dt.sqrt()).powf(2.0);

    // Calculate the probability of an up move and the probability of a down move
    let p_up = (u - d) / (u - d * d);
    let p_down = 1.0 - p_up;

    // Create a vector to hold the option prices at each step
    let mut prices = vec![0.0; (steps + 1) as usize];

    // Set the initial option price at each node
    for i in 0..=steps {
        let price = underlier_price * d.powf(i as f64) * u.powf((steps - i) as f64);

        if option_type == OptionType::Call {
            prices[i as usize] = price.max(price - strike_price);
        } else {
            prices[i as usize] = price.max(strike_price - price);
        }
    }

    // Iterate over the steps in reverse order
    for i in (1..=steps).rev() {
        // Calculate the discounted price at each node
        let discount = (-interest_rate * dt).exp();

        // Update the option prices using the lattice tree method
        for j in 0..i {
            let price = (p_up * prices[(j + 1) as usize] + p_down * prices[j as usize]) * discount;

            if option_type == OptionType::Call {
                prices[j as usize] = price.max(price - strike_price);
            } else {
                prices[j as usize] = price.max(strike_price - price);
            }
        }
    }
    // Return the option price at the root node
    prices[0]
}

// Price an option using the lattice method




fn option_price_binomial(underlier_price: f64, strike_price: f64, days: u32, volatility: f64, dividend_yield: f64, interest_rate: f64, option_type: OptionType) -> f64 {
    // Calculate the time step
    let dt = days as f64 / 365.0;

    // Calculate the up and down factors
    let u = (1.0 + volatility * dt.sqrt()).powf(2.0);
    let d = (1.0 - volatility * dt.sqrt()).powf(2.0);

    // Calculate the probability of an up move and the probability of a down move
    let p_up = (u - d) / (u - d * d);
    let p_down = 1.0 - p_up;

    // Create a vector to hold the option prices at each step
    let mut prices = vec![0.0; (days + 1) as usize];

    // Set the initial option price at each node
    for i in 0..=days {
        let price = underlier_price * d.powf(i as f64) * u.powf((days - i) as f64);

        if option_type == OptionType::Call {
            prices[i as usize] = price.max(price - strike_price);
        } else {
            prices[i as usize] = price.max(strike_price - price);
        }
    }

    // Iterate over the days in reverse order
    for i in (1..=days).rev() {
        // Calculate the discounted price at each node
        let discount = (-interest_rate * dt).exp();

        // Update the option prices using the binomial model
        for j in 0..i {
            let price = (p_up * prices[(j + 1) as usize] + p_down * prices[j as usize]) * discount;

            if option_type == OptionType::Call {
                prices[j as usize] = price.max(price - strike_price);
            } else {
                prices[j as usize] = price.max(strike_price - price);
            }
        }
    }

    // Return the option price at the root node
    prices[0]
}

fn file_download() {
    let url = "ftp://ftp.nasdaqtrader.com/symboldirectory/";
    let files = [ "nasdaqlisted.txt", "otherlisted"];
    let mut ftp_stream = FtpStream::connect("ftp://ftp.nasdaqtrader.com/symboldirectory/").unwrap();
    let _ = ftp_stream.login("", "").unwrap();
    println!("Current directory: {}", ftp_stream.pwd().unwrap());

    // Retrieve (GET) a file from the FTP server in the current working directory.
    let remote_file = ftp_stream.simple_retr(files[0]).unwrap();
    println!("Read file with contents\n{}\n", std::str::from_utf8(&remote_file.into_inner()).unwrap());

    // Store (PUT) a file from the client to the current working directory of the server.
    let mut reader = Cursor::new("Hello from the Rust \"ftp\" crate!".as_bytes());
    let _ = ftp_stream.put("greeting.txt", &mut reader);
    println!("Successfully wrote greeting.txt");

    // Terminate the connection to the server.
    let _ = ftp_stream.quit();
    // return reader;
}



// Enum to represent the option type
#[derive(PartialEq)]
enum OptionType {
    Call,
    Put
}

#[tokio::main]
pub async fn main() {
    // see if any command line arguments were provided (tickers or option contract identifiers can be passed in at the command line)
    interactive().await;
    // file_download();

    // let call_price = option_price_lattice(389.0, 390.0, 0.2, 0.05, 0.01, 0.025, 1000, OptionType::Call);
    // let put_price = option_price_lattice(389.0, 391.0, 0.2, 0.05, 0.01, 0.025, 1000, OptionType::Put);
    // let call_price = option_price_black_scholes(389.0, 390.0, 5, 0.25, 0.05, 0.0355, OptionType::Call);
    // let put_price = option_price_black_scholes(389.0, 390.0, 5, 0.25, 0.05, 0.0355, OptionType::Put);

    // println!("Call price: {}, put price: {}", call_price, put_price);

    // sandbox().await;
    // xyz().await;
}
