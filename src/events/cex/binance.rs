use crate::config;
use crate::events;
use crate::events::cex;
use log::error;
use serde::Deserialize;
use std::sync::Arc;
use std::time::Duration;

pub struct Binance {
    pub client: reqwest::Client,
    pub config: Arc<config::Config>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Coin {
    pub symbol: String,
    pub last_price: String,
    pub quote_volume: String,
}

#[async_trait::async_trait]
impl cex::Api for Binance {
    async fn get_ticker(&self) -> Result<Vec<cex::Coin>, events::Error> {
        let body = self
            .client
            .get(&self.config.binance_url)
            .timeout(Duration::from_secs(3))
            .send()
            .await?
            .text()
            .await?;

        let res: Vec<Coin> = serde_json::from_str(&body)?;

        let mut res_cex = vec![];

        for i in res {
            res_cex.push(cex::Coin {
                symbol: i.symbol,
                last_price: match i.last_price.parse::<f64>() {
                    Ok(val) => val,
                    Err(err) => {
                        error!("Unable to parse f64 from string: {err}");
                        return Err(events::Error::Other(Box::new(err)));
                    }
                },
                quote_volume: match i.quote_volume.parse::<f64>() {
                    Ok(val) => val,
                    Err(err) => {
                        error!("Unable to parse f64 from string: {err}");
                        return Err(events::Error::Other(Box::new(err)));
                    }
                },
            });
        }

        Ok(res_cex)
    }
}
