use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct TimeSale {
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

#[derive(Debug, Deserialize)]
pub struct Trade {
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

#[derive(Debug, Serialize, Deserialize)]
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


#[derive(Debug, Serialize, Deserialize)]
pub struct Summary {
    #[serde(rename = "type")]
    type_: String,
    symbol: String,
    open: String,
    high: String,
    low: String,
    close: Option<String>,
    prevClose: Option<String>  
}

  
#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum GenericResponseResult {    
    Trade(Trade),
    Quote(Quote),
    TimeSale(TimeSale),
    Summary(Summary)
}

#[derive(Deserialize, Serialize, Debug)]
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