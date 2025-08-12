use crate::events;
use std::sync::Arc;

pub mod binance;
pub mod bybit;
pub mod gate;
pub mod kucoin;
pub mod mexc;

pub struct Cex {
    pub binance: Arc<binance::Binance>,
    pub bybit: Arc<bybit::Bybit>,
    pub gate: Arc<gate::Gate>,
    pub kucoin: Arc<kucoin::Kucoin>,
    pub mexc: Arc<mexc::Mexc>,
}

#[async_trait::async_trait]
pub trait Api: Send + Sync {
    async fn get_ticker(&self) -> Result<Vec<Coin>, events::Error>;
}

#[derive(Clone, Debug)]
pub struct Coin {
    pub symbol: String,
    pub last_price: f64,
    pub quote_volume: f64,
}
