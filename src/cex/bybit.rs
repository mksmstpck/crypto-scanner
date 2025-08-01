use std::time::Duration;

use serde::Deserialize;

use crate::cex;
use log::error;

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
impl cex::Api for Bybit {
    async fn get_ticker(&self) -> Result<Vec<cex::Coin>, cex::Error> {
        let body = self
            .client
            .get("https://api.bybit.com/v5/market/tickers?category=spot")
            .timeout(Duration::from_secs(3))
            .send()
            .await?
            .text()
            .await?;

        let res: Response = serde_json::from_str(&body)?;

        let mut res_cex = vec![];

        for i in res.result.list {
            res_cex.push(cex::Coin {
                symbol: i.symbol,
                last_price: match i.last_price.parse::<f64>() {
                    Ok(val) => val,
                    Err(err) => {
                        error!("Unable to parse f64 from string: {err}");
                        0.0;
                        return Err(cex::Error::Other(Box::new(err)));
                    }
                },
                quote_volume: match i.turnover24h.parse::<f64>() {
                    Ok(val) => val,
                    Err(err) => {
                        error!("Unable to parse f64 from string: {err}");
                        0.0;
                        return Err(cex::Error::Other(Box::new(err)));
                    }
                },
            });
        }

        Ok(res_cex)
    }
}
