use crate::events;
use crate::events::cex;
use crate::services;
use std::collections::HashMap;
use std::error::Error as StdError;
use std::sync::Arc;

use crate::services::Services;

impl Services {
    pub async fn seek_pairs(&mut self) -> Result<Vec<services::Pair>, events::Error> {
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

                let pair = services::Pair {
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
    ) -> Result<Vec<services::Filtered>, Box<dyn StdError + Send + Sync>> {
        let all_coingecko_coins = self.coingecko.get_all_coins_list().await?;

        let exchanges_apis: Vec<(services::Exchange, Arc<dyn cex::Api>)> = vec![
            (
                services::Exchange::Binance,
                Arc::new(cex::binance::Binance {
                    client: self.client.clone(),
                    config: self.config.clone(),
                }),
            ),
            (
                services::Exchange::Bybit,
                Arc::new(cex::bybit::Bybit {
                    client: self.client.clone(),
                    config: self.config.clone(),
                }),
            ),
            (
                services::Exchange::Gate,
                Arc::new(cex::gate::Gate {
                    client: self.client.clone(),
                    config: self.config.clone(),
                }),
            ),
            (
                services::Exchange::Kucoin,
                Arc::new(cex::kucoin::Kucoin {
                    client: self.client.clone(),
                    config: self.config.clone(),
                }),
            ),
            (
                services::Exchange::Mexc,
                Arc::new(cex::mexc::Mexc {
                    client: self.client.clone(),
                    config: self.config.clone(),
                }),
            ),
        ];

        let mut grouped: HashMap<String, Vec<services::Price>> = HashMap::new();

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
                        if let Some(coin_id) = self
                            .coingecko
                            .resolve_coin_id(&coin.symbol, &all_coingecko_coins)
                        {
                            grouped
                                .entry(coin_id.clone())
                                .or_default()
                                .push(services::Price {
                                    exchange: exchange_enum,
                                    price: coin.last_price,
                                });
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error from {}: {}", exchange_enum, e);
                }
            }
        }

        let filtered: Vec<services::Filtered> = grouped
            .into_iter()
            .filter(|(_, prices)| prices.len() > 1)
            .map(|(coin, prices)| services::Filtered { coin, prices })
            .collect();

        Ok(filtered)
    }
}
