use std::{error::Error, time::Duration};

use crate::{dto, models};

pub struct Mexc {
    client: reqwest::Client,
}

type Response = Vec<models::Coin>;

impl Mexc {
    pub fn new(client: reqwest::Client) -> Mexc {
        Mexc { client }
    }

    pub async fn get_ticker(self) -> Result<Vec<dto::Coin>, Box<dyn Error>> {
        let body = self
            .client
            .get("https://api.mexc.com/api/v3/ticker/24hr")
            .timeout(Duration::from_secs(3))
            .send()
            .await?
            .text()
            .await?;

        println!("{:#?}", body);

        let res: Response = serde_json::from_str(&body)?;

        models::into_cex_response_dto(res)
    }

    pub async fn get_ticker_coin(self, symbol: &str) -> Result<dto::Coin, Box<dyn Error>> {
        let body = self
            .client
            .get(format!(
                "{}{}",
                "https://api.mexc.com/api/v3/ticker/24hr?symbol=", symbol
            ))
            .timeout(Duration::from_secs(3))
            .send()
            .await?
            .text()
            .await?;

        let res: models::Coin = serde_json::from_str(&body)?;

        res.into_coin_dto()
    }
}
