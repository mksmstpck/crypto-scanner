use crate::cex;
use crate::config;
use crate::storage;
use std::collections::HashMap;
use std::error::Error as StdError;
use std::sync::Arc;
use std::vec;

pub struct Services {
    pub stor: storage::Storage,
    pub config: Arc<config::Config>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Exchange {
    Binance,
    Bybit,
    Gate,
    Kucoin,
    Mexc,
}

impl std::fmt::Display for Exchange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone)]
pub struct Price {
    pub exchange: Exchange,
    pub price: f64,
}

#[derive(Debug)]
pub struct Filtered {
    coin: String,
    prices: Vec<Price>,
}

pub struct ExchangeClient {
    pub exchange: Exchange,
    pub client: Arc<dyn cex::Api>,
}

#[derive(Clone, Debug)]
pub struct Pair {
    pub price_high: Price,
    pub price_low: Price,
    pub coin: String,
    pub spread_percents: f64,
}

impl Services {
    pub fn new(stor: storage::Storage, config: Arc<config::Config>) -> Services {
        Services { stor, config }
    }

    // returns coins with more than 1.5% spread
    pub async fn seek_pairs(&mut self) -> Result<Vec<Pair>, cex::Error> {
        let filtered = self.scan_all_exchanges().await?;

        let mut pairs = vec![];

        for i in filtered {
            let max_price = i
                .prices
                .iter()
                .cloned()
                .max_by(|a, b| a.price.partial_cmp(&b.price).unwrap());
            let min_price = i
                .prices
                .iter()
                .cloned()
                .min_by(|a, b| a.price.partial_cmp(&b.price).unwrap());

            if let (Some(max), Some(min)) = (max_price, min_price) {
                if min.price == 0.0 {
                    continue;
                }

                let spread = ((max.price - min.price) / min.price) * 100.0;

                let pair = Pair {
                    price_high: max,
                    price_low: min,
                    coin: i.coin.clone(),
                    spread_percents: spread,
                };

                if spread > self.config.min_pair_ratio {
                    pairs.push(pair);
                }
            }
        }

        let mut new_pairs = vec![];

        for i in pairs {
            if !self.stor.check_exists(&i).await {
                new_pairs.push(i.clone());
                self.stor.put(i).await?;
            }
        }

        Ok(new_pairs)
    }

    async fn scan_all_exchanges(
        &mut self,
    ) -> Result<Vec<Filtered>, Box<dyn StdError + Send + Sync>> {
        let client = reqwest::Client::new();

        let exchanges_apis: Vec<(Exchange, Arc<dyn cex::Api>)> = vec![
            (
                Exchange::Binance,
                Arc::new(cex::binance::Binance {
                    client: client.clone(),
                    config: self.config.clone(),
                }),
            ),
            (
                Exchange::Bybit,
                Arc::new(cex::bybit::Bybit {
                    client: client.clone(),
                    config: self.config.clone(),
                }),
            ),
            (
                Exchange::Gate,
                Arc::new(cex::gate::Gate {
                    client: client.clone(),
                    config: self.config.clone(),
                }),
            ),
            (
                Exchange::Kucoin,
                Arc::new(cex::kucoin::Kucoin {
                    client: client.clone(),
                    config: self.config.clone(),
                }),
            ),
            (
                Exchange::Mexc,
                Arc::new(cex::mexc::Mexc {
                    client: client.clone(),
                    config: self.config.clone(),
                }),
            ),
        ];

        let mut grouped: HashMap<String, Vec<Price>> = HashMap::new();

        let futures: Vec<_> = exchanges_apis
            .into_iter()
            .map(|(exchange_enum, exchange_api)| async move {
                let result = exchange_api.get_ticker().await;
                (exchange_enum, result)
            })
            .collect();

        for (exchange_enum, result) in futures::future::join_all(futures).await {
            match result {
                Ok(coins) => {
                    for coin in coins {
                        grouped.entry(coin.symbol.clone()).or_default().push(Price {
                            exchange: exchange_enum,
                            price: coin.last_price,
                        });
                    }
                }
                Err(e) => {
                    eprintln!("Error from {}: {}", exchange_enum, e);
                }
            }
        }

        let filtered: Vec<Filtered> = grouped
            .into_iter()
            .filter(|(_, prices)| prices.len() > 1)
            .map(|(coin, prices)| Filtered { coin, prices })
            .collect();

        Ok(filtered)
    }
}
