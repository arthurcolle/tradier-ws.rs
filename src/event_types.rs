mod primitives;

#[derive(Debug, Deserialize, Serialize, Copy, Clone)]
pub struct TimeSale {
  type_: String,
  symbol: String,
  exch: String,
  bid: String,
  ask: String,
  last: String,
  size: String,
  date: String  
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Trade {
  type_: String,
  symbol: String,
  exch: String,
  price: f64,
  size: i64,
  cvol: i64,
  date: i64,
  last: f64
}

#[derive(Debug, Deserialize, Serialize)]
struct GenericResponse {
    ok: bool,
    error_code: u32,
    description: String,
    result: GenericResponseResult
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
enum GenericResponseResult {
    Trade(Trade),
    Quote(Quote),
    TimeSale(TimeSale),
    Summary(Summary)
}

pub struct Quote {
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
pub struct Quote {
  type_: String,
  symbol: String,
  exch: String,
  bid: f32,
  bidsz: i32,
  biddate: i64,
  askdate: i64,
  ask: f32,
  asksz: i32,
  last: f32,
  size: i32,
  date: i64,
  cvol: i64
}

pub struct Summary {
  type_: String,
  symbol: String,
  open: String,
  high: String,
  low: String,
  prevClose: String,
  close: String
}

pub enum MsgType {
  Trade, Quote, Summary, TimeSale
}

// {
//    "type":"trade",
//    "symbol":"SPY",
//    "exch":"P",
//    "price":"399.9",
//    "size":"0",
//    "cvol":"60429025",
//    "date":"1669165200001",
//    "last":"399.9"
// }

// {"type":"timesale","symbol":"SPY","exch":"P","bid":"407.14","ask":"407.17","last":"407.15","size":"100","date":"1669931173186"}

pub struct TimeSale {
  type_: String,
  symbol: String,
  exch: String,
  bid: String,
  ask: String,
  last: String,
  size: String,
  date: String
}

pub async fn xyz() -> ! {
  let x: String = r#"
    {"type":"timesale","symbol":"SPY","exch":"P","bid":"407.12","ask":"407.15","last":"407.15","size":"600","date":"1669932568450"}
  "#;
}