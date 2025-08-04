use crate::cex;
use log::error;
use serde::Deserialize;
use std::time::Duration;
use crate::config;
use std::sync::Arc;

pub struct Binance {
    pub client: reqwest::Client,
    pub config: Arc<config::Config>
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
    async fn get_ticker(&self) -> Result<Vec<cex::Coin>, cex::Error> {
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
                        return Err(cex::Error::Other(Box::new(err)));
                    }
                },
                quote_volume: match i.quote_volume.parse::<f64>() {
                    Ok(val) => val,
                    Err(err) => {
                        error!("Unable to parse f64 from string: {err}");
                        return Err(cex::Error::Other(Box::new(err)));
                    }
                },
            });
        }

        Ok(res_cex)
    }
}
