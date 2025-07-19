use std::time::Duration;

use crate::events::cex::{self, CexApi};
use crate::models::{self, into_cex_response_dto};

use crate::dto;

type Response = Vec<models::Coin>;

pub struct Binance {
    pub client: reqwest::Client,
}

#[async_trait::async_trait]
impl CexApi for Binance {
    async fn get_ticker(&self) -> Result<Vec<dto::Coin>, cex::CexError> {
        let body = self
            .client
            .get("https://api.binance.com/api/v3/ticker/24hr")
            .timeout(Duration::from_secs(3))
            .send()
            .await?
            .text()
            .await?;

        let res: Response = serde_json::from_str(&body)?;

        into_cex_response_dto(res)
    }

    /* pub async fn get_ticker_coin(self, symbol: &str) -> Result<dto::Coin, Box<dyn Error>> {
        let body = self
            .client
            .get(format!(
                "{}{}",
                "https://api.binance.com/api/v3/ticker/24hr?symbol=", symbol
            ))
            .timeout(Duration::from_secs(3))
            .send()
            .await?
            .text()
            .await?;

        let res: models::Coin = serde_json::from_str(&body)?;

        res.into_coin_dto()
    }*/
}
