use std::{error::Error, time::Duration};

use crate::events::{models};

pub struct Gate {
    client: reqwest::Client,
}

type Response = Vec<models::Coin>;

impl Gate {
    pub fn new(client: reqwest::Client) -> Gate {
        Gate { client }
    }

    pub async fn get_ticker(self) -> Result<Response, Box<dyn Error>> {
        let body = self
            .client
            .get("https://api.gate.io/api/v4/spot/tickers")
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
        .get(format!("{}{}", "https://api.gate.io/api/v4/spot/tickers?currency_pair=", symbol))
        .timeout(Duration::from_secs(3))
        .send()
        .await?
        .text()
        .await?;

        let res: models::Coin = serde_json::from_str(&body)?;

        Ok(res)
    }
}
