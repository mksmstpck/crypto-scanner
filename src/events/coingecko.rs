use crate::{config, events};
use log::error;
use serde::Deserialize;
use std::sync::Arc;
use std::time::Duration;

pub struct Coingecko {
    pub config: Arc<config::Config>,
    pub client: reqwest::Client,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CoinMarkets {
    pub id: String,
    pub total_volume: f64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Coin {
    pub id: String,
    pub name: String,
    pub tickers: Vec<Ticker>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Ticker {
    pub base: String,
    pub target: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Market {
    pub name: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CoinListItem {
    pub id: String,
    pub symbol: String,
    pub name: String,
}

impl Coingecko {
    pub fn new(config: Arc<config::Config>, client: reqwest::Client) -> Self {
        Coingecko { config, client }
    }

    pub async fn get_coin_markets(&self) -> Result<Vec<CoinMarkets>, events::Error> {
        let mut all_coin_markets: Vec<CoinMarkets> = Vec::new();
        let mut page_number = 1;
        const MAX_PAGES: u32 = 20;

        loop {
            let url = format!("{}{}", self.config.coingecko_bulk_data, page_number);

            let body = self
                .client
                .get(&url)
                .timeout(Duration::from_secs(10))
                .send()
                .await?
                .text()
                .await?;

            let current_page_data: Vec<CoinMarkets> = serde_json::from_str(&body)?;

            if current_page_data.is_empty() {
                break;
            }

            all_coin_markets.extend(current_page_data);

            page_number += 1;

            if page_number > MAX_PAGES {
                error!(
                    "Reached max pages ({}) for CoinGecko API. Breaking loop.",
                    MAX_PAGES
                );
                break;
            }

            tokio::time::sleep(Duration::from_millis(500)).await;
        }

        Ok(all_coin_markets)
    }

    pub async fn get_coin_info(&self, id: String) -> Result<Coin, events::Error> {
        let body = self
            .client
            .get(format!("{}{}", &self.config.coingecko_coin, id))
            .timeout(Duration::from_secs(3))
            .send()
            .await?
            .text()
            .await?;

        let res: Coin = serde_json::from_str(&body)?;

        Ok(res)
    }

    pub async fn get_all_coins_list(&self) -> Result<Vec<CoinListItem>, events::Error> {
        let url = format!("{}coins/list", self.config.coingecko_base);
        let body = self
            .client
            .get(&url)
            .timeout(Duration::from_secs(10))
            .send()
            .await?
            .text()
            .await?;

        let res: Vec<CoinListItem> = serde_json::from_str(&body)?;
        Ok(res)
    }

    pub fn resolve_coin_id(&self, symbol: &str, all_coins: &[CoinListItem]) -> Option<String> {
        let lower_symbol = symbol.to_lowercase();
        all_coins
            .iter()
            .find(|c| c.symbol.to_lowercase() == lower_symbol)
            .map(|c| c.id.clone())
    }
}
