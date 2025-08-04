use std::time::Duration;

use crate::{cex, config};
use log::error;
use std::sync::Arc;

pub struct Gate {
    pub client: reqwest::Client,
    pub config: Arc<config::Config>,
}

#[derive(serde::Deserialize)]
struct Coin {
    currency_pair: String,
    last: String,
    quote_volume: String,
}

#[async_trait::async_trait]
impl cex::Api for Gate {
    async fn get_ticker(&self) -> Result<Vec<cex::Coin>, cex::Error> {
        let body = self
            .client
            .get(&self.config.gate_url)
            .header("User-Agent", "Mozilla/5.0 ArbitrageBot")
            .timeout(Duration::from_secs(10))
            .send()
            .await?
            .text()
            .await?;

        let res: Vec<Coin> = serde_json::from_str(&body)?;

        let mut res_cex = vec![];

        for i in res {
            res_cex.push(cex::Coin {
                symbol: i.currency_pair,
                last_price: match i.last.parse::<f64>() {
                    Ok(val) => val,
                    Err(err) => {
                        error!("Unable to parse f64 from string: {err}");
                        0.0;
                        return Err(cex::Error::Other(Box::new(err)));
                    }
                },
                quote_volume: match i.quote_volume.parse::<f64>() {
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
