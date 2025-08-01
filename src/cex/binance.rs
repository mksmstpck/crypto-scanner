use crate::cex;
use log::error;
use serde::Deserialize;
use std::time::Duration;

pub struct Binance {
    pub client: reqwest::Client,
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
            .get("https://api.binance.com/api/v3/ticker/24hr")
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
                        0.0;
                        return Err(cex::Error::Other(err.to_string()));
                    }
                },
                quote_volume: match i.quote_volume.parse::<f64>() {
                    Ok(val) => val,
                    Err(err) => {
                        error!("Unable to parse f64 from string: {err}");
                        0.0;
                        return Err(cex::Error::Other(err.to_string()));
                    }
                },
            });
        }

        Ok(res_cex)
    }
}
