use std::{error::Error, time::Duration};

use crate::events::models::{self, Coin};

pub struct Kucoin {
    client: reqwest::Client,
}

type Ticker = Vec<Coin>;

#[derive(serde::Deserialize)]
struct Response {
    ticker: Ticker
}

impl Kucoin {
    pub fn new(client: reqwest::Client) -> Kucoin {
        Kucoin { client }
    }

    pub async fn get_ticker(self) -> Result<Response, Box<dyn Error>> {
        let body = self
            .client
            .get("https://api.kucoin.com/api/v1/market/allTickers")
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
        .get(format!("{}{}", "https://api.kucoin.com/api/v1/market/stats?symbol=", symbol))
        .timeout(Duration::from_secs(3))
        .send()
        .await?
        .text()
        .await?;

        let res: models::Coin = serde_json::from_str(&body)?;

        Ok(res)
    }
}
