use crate::config;
use crate::events;
use crate::storage;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct Services {
    pub stor: storage::Storage,
    pub config: Arc<config::Config>,
    pub coingecko: events::coingecko::Coingecko,
    pub coin_cache: CoinCache,
    pub client: reqwest::Client,
}

#[derive(Debug, Clone)]
pub struct CoinInfo {
    pub id: String,
    pub symbol: String,
    pub name: String,
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

#[derive(Debug, Clone)]
pub struct Filtered {
    pub coin: String,
    pub prices: Vec<Price>,
}

pub struct ExchangeClient {
    pub exchange: Exchange,
    pub client: Arc<dyn events::cex::Api>,
}

#[derive(Clone, Debug)]
pub struct Pair {
    pub price_high: Price,
    pub price_low: Price,
    pub coin: String,
    pub spread_percents: f64,
}

impl Services {
    pub fn new(
        stor: storage::Storage,
        config: Arc<config::Config>,
        coingecko: events::coingecko::Coingecko,
        client: reqwest::Client,
    ) -> Services {
        Services {
            stor,
            config,
            coingecko,
            coin_cache: CoinCache::new(),
            client,
        }
    }
}

#[derive(Clone)]
pub struct CoinCache {
    pub coins: Arc<RwLock<Vec<events::coingecko::CoinListItem>>>,
}

impl CoinCache {
    pub fn new() -> Self {
        CoinCache {
            coins: Arc::new(RwLock::new(Vec::new())),
        }
    }
}

pub mod cex;
pub mod coingecko;
