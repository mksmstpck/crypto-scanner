use std::time::Duration;

use crate::{dto, models};

use crate::events::cex;

pub struct Mexc {
    pub client: reqwest::Client,
}

type Response = Vec<models::Coin>;

#[async_trait::async_trait]
impl cex::CexApi for Mexc {
    async fn get_ticker(&self) -> Result<Vec<dto::Coin>, cex::CexError> {
        let body = self
            .client
            .get("https://api.mexc.com/api/v3/ticker/24hr")
            .timeout(Duration::from_secs(3))
            .send()
            .await?
            .text()
            .await?;

        let res: Response = serde_json::from_str(&body)?;

        models::into_cex_response_dto(res)
    }

    /*pub async fn get_ticker_coin(self, symbol: &str) -> Result<dto::Coin, Box<dyn Error>> {
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
    }*/
}
