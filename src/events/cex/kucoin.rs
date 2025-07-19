use serde::Deserialize;
use std::time::Duration;

use crate::{
    dto,
    events::cex,
    models::{self},
};

pub struct Kucoin {
    pub client: reqwest::Client,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Coin {
    symbol: String,
    vol_value: String,
    last: String,
}

type Ticker = Vec<Coin>;

#[derive(serde::Deserialize)]
struct Data {
    ticker: Ticker,
}

#[derive(serde::Deserialize)]
struct Response {
    data: Data,
}

#[async_trait::async_trait]
impl cex::CexApi for Kucoin {
    async fn get_ticker(&self) -> Result<Vec<dto::Coin>, cex::CexError> {
        let body = self
            .client
            .get("https://api.kucoin.com/api/v1/market/allTickers")
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

        for i in res.data.ticker {
            res_models.push(models::Coin {
                symbol: i.symbol,
                last_price: i.last,
                quote_volume: i.vol_value,
            });
        }

        models::into_cex_response_dto(res_models)
    }
}
