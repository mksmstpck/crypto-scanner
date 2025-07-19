use std::time::Duration;

use crate::dto;
use crate::events::cex;
use crate::models;

pub struct Gate {
    pub client: reqwest::Client,
}

#[derive(serde::Deserialize)]
struct Coin {
    currency_pair: String,
    last: String,
    quote_volume: String,
}

#[async_trait::async_trait]
impl cex::CexApi for Gate {
    async fn get_ticker(&self) -> Result<Vec<dto::Coin>, cex::CexError> {
        let body = self
            .client
            .get("https://api.gateio.ws/api/v4/spot/tickers")
            .header("User-Agent", "Mozilla/5.0 ArbitrageBot")
            .timeout(Duration::from_secs(10))
            .send()
            .await?
            .text()
            .await?;

        let res: Vec<Coin> = serde_json::from_str(&body)?;

        let mut res_models = vec![models::Coin {
            symbol: String::from(""),
            last_price: String::from(""),
            quote_volume: String::from(""),
        }];

        for i in res {
            res_models.push(models::Coin {
                symbol: i.currency_pair,
                last_price: i.last,
                quote_volume: i.quote_volume,
            });
        }

        models::into_cex_response_dto(res_models)
    }
}
