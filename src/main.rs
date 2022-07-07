use reqwest::Client;
use rocket::serde::{json::Json, Deserialize, Serialize};
use serde_json::{json, Error, Value};
use std::fmt::Debug;
extern crate dotenv;
use dotenv::dotenv;
use std::env;

#[derive(Debug, Serialize, Deserialize)]
struct Ticker {
    pair: String,
    bid: f64,
    ask: f64,
    last_trade: f64, //luno
    exchange: String,
}

impl Ticker {
    fn new(pair: String, bid: f64, ask: f64, last_trade: f64, exchange: String) -> Ticker {
        Ticker {
            pair,
            bid,
            ask,
            last_trade,
            exchange,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
// struct BitfinexResp(String, u64, f64, u64, f64, f64, f64, u64, f64, u64, f64);
struct BitfinexResp(f64, f64, f64, f64, f64, f64, f64, f64, f64, f64);

#[derive(Debug, Serialize, Deserialize)]
struct LunoResp {
    pair: String,
    timestamp: usize, //luno
    bid: String,
    ask: String,
    last_trade: String,             //luno
    rolling_24_hour_volume: String, //luno
    status: String,                 //luno
}

#[derive(Debug, Serialize, Deserialize)]
struct AAResp {
    #[serde(rename = "1. From_Currency Code")]
    from: String,
    #[serde(rename = "2. From_Currency Name")]
    from_name: String,
    #[serde(rename = "3. To_Currency Code")]
    to: String,
    #[serde(rename = "4. To_Currency Name")]
    to_name: String,
    #[serde(rename = "5. Exchange Rate")]
    exchange_rate: String,
    #[serde(rename = "6. Last Refreshed")]
    last_refreshed: String,
    #[serde(rename = "7. Time Zone")]
    time_zone: String,
    #[serde(rename = "8. Bid Price")]
    bid: String,
    #[serde(rename = "9. Ask Price")]
    ask: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct AA {
    #[serde(rename = "Realtime Currency Exchange Rate")]
    data: AAResp,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MyError {
    Reqwest {
        status: i32,
        error: String,
        help: String,
    },
    SerdeJson {
        error: String,
    },
    EmptyError,
    NewError {
        error: String,
        help: String,
    },
}

impl From<reqwest::Error> for MyError {
    fn from(error: reqwest::Error) -> Self {
        MyError::Reqwest {
            status: 500,
            error: error.to_string(),
            help: "it looks like the ticker does not exist. Try XBT".to_string(),
        }
    }
}

impl From<serde_json::Error> for MyError {
    fn from(error: serde_json::Error) -> Self {
        MyError::SerdeJson {
            error: error.to_string(),
        }
    }
}

async fn zar_price(sym: &str) -> Result<Ticker, MyError> {
    let client = Client::builder().use_rustls_tls().build()?;
    let luno_resp: LunoResp = client
        .get(format!(
            "https://api.luno.com/api/1/ticker?pair={}",
            sym.to_uppercase()
        ))
        .send()
        .await?
        .json()
        .await?;

    Ok(Ticker::new(
        luno_resp.pair.to_string(),
        luno_resp.bid.parse::<f64>().unwrap(),
        luno_resp.ask.parse::<f64>().unwrap(),
        luno_resp.last_trade.parse::<f64>().unwrap(),
        "Luno".to_string(),
    ))
}

async fn usd_zar(from: &str, to: &str) -> Result<Ticker, MyError> {
    dotenv().unwrap();
    let apikey = env::var("A").expect("missing alphaVantage api secret in .env file");
    // let client = Client::new();
    let client = Client::builder().use_rustls_tls().build()?;
    let aa_resp: AA = client
        .get(format!("https://www.alphavantage.co/query?function=CURRENCY_EXCHANGE_RATE&from_currency={}&to_currency={}&apikey={}", from.to_uppercase(), to.to_uppercase(), apikey))
        .send()
        .await?
        .json()
        .await?;

    Ok(Ticker::new(
        format!("{}{}", from.to_uppercase(), to.to_uppercase()),
        aa_resp.data.bid.parse::<f64>().unwrap(),
        aa_resp.data.ask.parse::<f64>().unwrap(),
        aa_resp.data.exchange_rate.parse::<f64>().unwrap(),
        "AlphaVantage".to_string(),
    ))
}

async fn usd_price(sym: &str) -> Result<Ticker, MyError> {
    // let client = Client::new();
    let client = Client::builder().use_rustls_tls().build()?;
    let resp = client
        .get(format!(
            "https://api-pub.bitfinex.com/v2/ticker/t{}",
            sym.to_uppercase()
        ))
        .send()
        .await?
        .text()
        .await?;

    let v: BitfinexResp = serde_json::from_str(&resp)?;

    Ok(Ticker::new(
        format!("{}", sym.to_uppercase()),
        v.0 as f64,
        v.2 as f64,
        v.8 as f64,
        "BitFinex".to_string(),
    ))
}

#[macro_use]
extern crate rocket;
#[get("/crypto/<symbol>")]
async fn crypto(symbol: &str) -> Result<Json<Ticker>, Json<MyError>> {
    let currency = &symbol[symbol.len() - 3..].to_uppercase();

    match currency.as_str() {
        "ZAR" => {
            let ticker = zar_price(&symbol.to_uppercase()).await;
            match ticker {
                Ok(ticker) => return Ok(Json(ticker)),
                Err(err) => Err(Json(err)),
            }
        }
        "USD" => {
            let ticker = usd_price(&symbol.to_uppercase()).await;
            match ticker {
                Ok(ticker) => return Ok(Json(ticker)),
                Err(err) => Err(Json(err)),
            }
        }
        _ => {
            return Err(Json(MyError::NewError {
                error: "failed to match the last three digits of the ticker".to_string(),
                help: "Try BTCUSD or XBTZAR".to_string(),
            }))
        }
    }
}

#[get("/forrex/<from>/<to>")]
async fn forrex(from: &str, to: String) -> Result<Json<Ticker>, Json<MyError>> {
    let ticker = usd_zar(&from.to_uppercase(), &to.to_uppercase()).await;

    match ticker {
        Ok(ticker) => return Ok(Json(ticker)),
        Err(err) => return Err(Json(err)),
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![crypto])
        .mount("/", routes![forrex])
}

// #[tokio::main]
// async fn main() {
//     let usdzar = usd_zar("usd", "ZAR").await;
//     let btczar = zar_price("xbtzar").await;
//     let btcusd = usd_price("btcusd").await;
//     println!("{:#?}", usdzar);
//     println!("{:#?}", btczar);
//     println!("{:#?}", btcusd);
// }
