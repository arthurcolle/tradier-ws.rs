use std::collections::HashMap;
use reqwest::header::{AUTHORIZATION, CONTENT_LENGTH, ACCEPT};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value, json};
use std::fmt;
use url::Url;
use tungstenite::{connect, Message};
use substring::Substring;
use websocket::ClientBuilder;

use std::io::stdin;
use std::sync::mpsc::channel;
use std::thread;


#[derive(Serialize, Deserialize, Debug)]
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
// struct TradierEvent {
//     type_: String,
//     symbol: String,
//     exch: String,
//     bid: f32,
//     bidsz: i32,
//     biddate: i64,
//     askdate: i64,
//     ask: f32,
//     asksz: i32,
//     last: f32,
//     size: i32,
//     date: i64,
//     cvol: i64
// }

struct TradierEvent {
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

struct Quote {
    type_: String,
    symbol: String,
    bid: f32,
    bidsz: i32,
    bidexch: String,
    biddate: i32
}

pub fn subscribe(payload: String) -> ! {
    let (mut socket, response) =
        connect(Url::parse("wss://ws.tradier.com/v1/markets/events").unwrap()).expect("Can't connect");

    println!("Connected to the server");
    println!("Response HTTP code: {}", response.status());
    println!("Response contains the following headers:");

    socket.write_message(Message::Text(payload));
    
    loop {
        let msg = socket.read_message().expect("Error reading message");
        println!("New msg: {:?}", msg);
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
    // println!("{}", access_token);
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

#[tokio::main]
#[warn(unused_variables)]
async fn main() {
    dotenv().ok();
    let tradier_access_token = std::env::var("TRADIER_API_KEY").expect("Access token not available!");
    let session_id = market_session(&tradier_access_token).await;
    let symbols: Vec<_> = ["SPY", "QQQ"].iter().map(|s| s.to_string()).collect();
    let payload = create_payload(symbols, session_id, true);
    subscribe(payload)
}
