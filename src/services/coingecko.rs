use crate::events;
use crate::events::coingecko;
use crate::services;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct CoinCache {
    pub coins: Arc<RwLock<Vec<events::coingecko::CoinListItem>>>,
}

impl services::Services {
    pub async fn get_all_coins_list(
        &self,
    ) -> Result<Vec<events::coingecko::CoinListItem>, events::Error> {
        let cached = self.coin_cache.coins.read().await;
        if !cached.is_empty() {
            return Ok(cached.clone());
        }
        drop(cached);

        let list = self.coingecko.get_all_coins_list().await?;
        let mut cache_write = self.coin_cache.coins.write().await;
        *cache_write = list.clone();
        Ok(list)
    }

    pub fn resolve_coin_id(
        &self,
        symbol: &str,
        all_coins: &[coingecko::CoinListItem],
    ) -> Option<String> {
        let lower_symbol = symbol.to_lowercase();
        all_coins
            .iter()
            .find(|c| c.symbol.to_lowercase() == lower_symbol)
            .map(|c| c.id.clone())
    }

    pub async fn filtered_market_coins_full(&self) -> Result<Vec<coingecko::Coin>, events::Error> {
        let all = self.coingecko.get_coin_markets().await?;
        let mut filtered = vec![];
        for i in all {
            if i.total_volume > 100_000.0
                && i.total_volume < 1_000_000.0
                && !i.id.contains("wrapped")
            {
                filtered.push(self.coingecko.get_coin_info(i.id).await?);
                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            }
        }
        Ok(filtered)
    }
}
