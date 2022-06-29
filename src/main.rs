use reqwest::{Client};
use serde::{Deserialize};
use rocket::serde::{Serialize, json::Json};
use thiserror::Error;
use rocket::http::Status;
use std::fmt::{Display, Debug};

#[derive(Debug, Serialize, Deserialize)]
struct Ticker {
    pair: String,
    timestamp: usize,
    bid: String,
    ask: String,
    last_trade: String,
    rolling_24_hour_volume: String,
    status: String
}

#[derive(Debug)]
pub enum MyError {
    Reqwest(reqwest::Error),
}

impl From<reqwest::Error> for MyError {
    fn from(error: reqwest::Error) -> Self {
        MyError::Reqwest(error)
    }
}

async fn zar_price(sym: String) -> Result<Ticker, MyError> {
    let client = Client::new();
    let luno_resp: Ticker = client.get(format!("https://api.luno.com/api/1/ticker?pair={sym}ZAR")).send().await?.json().await?;
    // println!("luno bid : {:?}", luno_resp.bid);
    Ok(luno_resp)
} 

#[macro_use] extern crate rocket;
#[get("/<symbol>")]
async fn index(symbol: String) -> Result<Json<Ticker>,String>{
    let ticker = zar_price(symbol.to_string().to_uppercase()).await;
    match ticker {
        Ok(resp) => Ok(Json(resp)),
    Err(a) =>  {
        // I would love to send better errors here
        println!("{a:?}");
        return Err("error in the symbol enpoint".to_string())
        },
    }
}

#[launch]
fn rocket () -> _ {
    rocket::build()
    .mount("/", routes![index])
}