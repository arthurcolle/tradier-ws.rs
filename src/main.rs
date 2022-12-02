use std::collections::BTreeMap;
use std::collections::HashMap;
use reqwest::header::{AUTHORIZATION, CONTENT_LENGTH, ACCEPT};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value, json};
use tokio::io::Stdout;
use std::fmt;
use std::io::{self, stdout};
use std::io::prelude::*;
use std::env::var;
use url::Url;
use tungstenite::{connect, Message};
use substring::Substring;
use websocket::ClientBuilder;

use std::io::stdin;
use std::sync::mpsc::channel;
use std::thread;
use std::fs;
use std::io::Write;

#[derive(Debug, Deserialize, Serialize)]
pub struct timesale {
  symbol: String,
  #[serde(rename = "type")]
  type_: String,
  exch: String,
  bid: String,
  ask: String,
  last: String,
  size: String,
  date: String  
}

#[derive(Debug, Deserialize, Serialize)]
pub struct trade {
  #[serde(rename = "type")]
  type_: String,
  symbol: String,
  exch: String,
  price: String,
  size: String,
  cvol: String,
  date: String,
  last: String
}

// fn s2f32 
#[derive(Debug, Serialize, Deserialize)]
pub struct quote {
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


#[derive(Debug, Serialize, Deserialize)]
pub struct summary {
    #[serde(rename = "type")]
    type_: String,
    symbol: String,
    open: String,
    high: String,
    low: String,
    close: String,
    prevClose: String  
}

  
#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum GenericResponseResult {    
    trade(trade),
    quote(quote),
    timesale(timesale),
    summary(summary)
}

//   example Quote payload
// {
//   "type":"quote", "symbol":"SPY",
//   "bid":399.84, "bidsz":6, "bidexch":"P", "biddate":"1669165196000",
//   "ask":399.87, "asksz":4, "askexch":"A", "askdate":"1669165200000"
// }


// {"type":"trade","symbol":"SPY","exch":"P","price":"407.38","size":"0","cvol":"75957622","date":"1669929000000","last":"407.38"}

// {"type":"quote","symbol":"SPY","bid":407.12,"bidsz":24,"bidexch":"P","biddate":"1669932556000","ask":407.16,"asksz":20,"askexch":"Q","askdate":"1669932568000"}

// {
//   "type":"summary",
//   "symbol":"SPY",
//   "open":"396.63",
//   "high":"400.07",
//   "low":"395.1527",
//   "prevClose":"394.59",
//   "close":"399.9"
// }

pub enum MsgType {
  Trade, Quote, Summary, TimeSale
}

#[derive(Deserialize, Serialize, Debug)]
struct MarketPayload {
    symbols: Vec<String>,
    sessionid: String,
    linebreak: bool
}

pub fn create_payload(symbols: Vec<String>, sessionid: String, linebreak: bool) -> String {
    let payload = MarketPayload {
        symbols: symbols.clone(),
        sessionid: sessionid.clone(),
        linebreak: linebreak.clone()
    };
    return json!(payload).to_string();
}

#[derive(Serialize, Deserialize, Debug)]

// struct GenericJson {
//
// }

struct Tradier {
    type_: String,
    symbol: String,
    exch: String,
    bid: String,
    bidsz: String,
    biddate: String,
    askdate: String,
    ask: String,
    asksz: String,
    last: String,
    size: String,
    date: String,
    cvol: String,
    price: String
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
        generic_parse(msg.to_string()).await;

        // let e: TradierEvent = serde_json::from_str(&msg.to_string()).unwrap();
        // println!("Parsed event: {:?}", e);
    }
}

impl fmt::Display for SessionDataL1 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(stream: {:?})", self.stream)
    }
}

#[derive(Serialize, Deserialize)]
pub struct SessionDataL1 {
    #[serde(flatten)]
    stream: HashMap<String, SessionDataL2>
}

impl fmt::Display for SessionDataL2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(url: {}, sessionid: {}", self.url, self.sessionid)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SessionDataL2 {
    url: String,
    sessionid: String
}

#[warn(unused_variables)]
pub async fn market_session(access_token: &str) -> String {
    let client = reqwest::Client::new();
    // println!("{}", access_token);
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

//# serde = { version = "1.0.99", features = ["derive"] }
//# serde_json = "1.0.40"

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
enum Data {
    A { value: Vec<u32> },
    B { value: Vec<Vec<u32>> },
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

    // let a: Data = serde_json::from_str(r#"{"type": "A", "value": [ 1, 2, 3, 4, 5 ]}"#).unwrap();
    // let b: Data = serde_json::from_str(r#"{"type": "B", "value": [[1, 2, 3, 4, 5], [6, 7, 8 ]]}"#).unwrap();

    // println!("{:?}", a);
    // println!("{:?}", b);
}

async fn generic_parse(variant: String) -> GenericResponseResult {
  let event: GenericResponseResult = serde_json::from_str(&variant).unwrap();
  println!("{:?}", event);
  return event;
}


pub async fn sandbox() {
    println!("Yo!")
}

#[tokio::main]
pub async fn main() {
    interactive().await;
    // sandbox().await;
    // xyz().await;
}
