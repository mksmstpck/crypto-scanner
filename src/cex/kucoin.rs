use crate::{cex, config};
use log::error;
use serde::Deserialize;
use std::sync::Arc;
use std::time::Duration;

pub struct Kucoin {
    pub client: reqwest::Client,
    pub config: Arc<config::Config>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Coin {
    symbol: Option<String>,
    vol_value: Option<String>,
    last: Option<String>,
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
impl cex::Api for Kucoin {
    async fn get_ticker(&self) -> Result<Vec<cex::Coin>, cex::Error> {
        let body = self
            .client
            .get(&self.config.kucoin_url)
            .timeout(Duration::from_secs(3))
            .send()
            .await?
            .text()
            .await?;

        let res: Response = serde_json::from_str(&body)?;

        let mut res_cex = vec![];

        for i in res.data.ticker {
            res_cex.push(cex::Coin {
                symbol: i.symbol.as_deref().unwrap_or("0").to_string(),
                last_price: match i.last.as_deref().unwrap_or("0").to_string().parse::<f64>() {
                    Ok(val) => val,
                    Err(err) => {
                        error!("Unable to parse f64 from string: {err}");
                        0.0;
                        return Err(cex::Error::Other(Box::new(err)));
                    }
                },
                quote_volume: match i
                    .vol_value
                    .as_deref()
                    .unwrap_or("0")
                    .to_string()
                    .parse::<f64>()
                {
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
