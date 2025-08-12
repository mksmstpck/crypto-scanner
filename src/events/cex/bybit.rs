use std::time::Duration;

use serde::Deserialize;

use crate::config;
use crate::events;
use crate::events::cex;
use log::error;
use std::sync::Arc;

pub struct Bybit {
    pub client: reqwest::Client,
    pub config: Arc<config::Config>,
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
    async fn get_ticker(&self) -> Result<Vec<cex::Coin>, events::Error> {
        let body = self
            .client
            .get(&self.config.bybit_url)
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
                        return Err(events::Error::Other(Box::new(err)));
                    }
                },
                quote_volume: match i.turnover24h.parse::<f64>() {
                    Ok(val) => val,
                    Err(err) => {
                        error!("Unable to parse f64 from string: {err}");
                        0.0;
                        return Err(events::Error::Other(Box::new(err)));
                    }
                },
            });
        }

        Ok(res_cex)
    }
}
