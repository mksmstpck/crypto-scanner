use std::time::Duration;

use serde::Deserialize;

use crate::events::cex;
use crate::models::{self};

use crate::dto;

pub struct Bybit {
    pub client: reqwest::Client,
}

#[derive(serde::Deserialize)]
struct Res {
    list: Vec<Coin>,
}

#[derive(serde::Deserialize)]
struct Response {
    result: Res,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Coin {
    pub symbol: String,
    pub last_price: String,
    pub turnover24h: String,
}

#[async_trait::async_trait]
impl cex::CexApi for Bybit {
    async fn get_ticker(&self) -> Result<Vec<dto::Coin>, cex::CexError> {
        let body = self
            .client
            .get("https://api.bybit.com/v5/market/tickers?category=spot")
            .timeout(Duration::from_secs(3))
            .send()
            .await?
            .text()
            .await?;

        let res: Response = serde_json::from_str(&body)?;

        let mut res_models = vec![models::Coin {
            symbol: String::from(""),
            last_price: String::from(""),
            quote_volume: String::from(""),
        }];

        for i in res.result.list {
            res_models.push(models::Coin {
                symbol: i.symbol,
                last_price: i.last_price,
                quote_volume: i.turnover24h,
            });
        }

        models::into_cex_response_dto(res_models)
    }

    /*pub async fn get_ticker_coin(self, symbol: &str) -> Result<dto::Coin, Box<dyn Error>> {
        let body = self
            .client
            .get(format!(
                "{}{}",
                "https://api.bybit.com/v5/market/tickers?category=spot&symbol=", symbol
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
