use std::{error::Error, time::Duration};

use crate::events::models::{self, Coin};

pub struct Bybit {
    client: reqwest::Client,
}

#[derive(serde::Deserialize)]
struct Res {
    list: Vec<Coin>
}

#[derive(serde::Deserialize)]
struct Response {
    result: Res
}

impl Bybit {
    pub fn new(client: reqwest::Client) -> Bybit {
        Bybit { client }
    }

    pub async fn get_ticker(self) -> Result<Response, Box<dyn Error>> {
        let body = self
            .client
            .get("https://api.bybit.com/v5/market/tickers?category=spot")
            .timeout(Duration::from_secs(3))
            .send()
            .await?
            .text()
            .await?;

        println!("{:#?}", body);

        let res: Response = serde_json::from_str(&body)?;

        Ok(res)
    }

    pub async fn get_ticker_coin(self, symbol:&str) -> Result<models::Coin, Box<dyn Error>> {
        let body = self.
        client
        .get(format!("{}{}", "https://api.bybit.com/v5/market/tickers?category=spot&symbol=", symbol))
        .timeout(Duration::from_secs(3))
        .send()
        .await?
        .text()
        .await?;

        let res: models::Coin = serde_json::from_str(&body)?;

        Ok(res)
    }
}
