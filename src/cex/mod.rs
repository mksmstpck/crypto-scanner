use std::sync::Arc;

use reqwest::{self};

use crate::{cex, config};

use std::error::Error as StdError;

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
    async fn get_ticker(&self) -> Result<Vec<cex::Coin>, Error>;
}

#[derive(Debug)]
pub enum Error {
    Http(reqwest::Error),
    Json(serde_json::Error),
    Config(config::Error),
    Other(Box<dyn StdError + Send + Sync>),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Http(e) => write!(f, "HTTP error: {e}"),
            Error::Json(e) => write!(f, "JSON error: {e}"),
            Error::Config(e) => write!(f, "Config error: {e}"),
            Error::Other(e) => write!(f, "Other error: {e}"),
        }
    }
}

impl std::error::Error for Error {}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::Http(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::Json(e)
    }
}

impl From<config::Error> for Error {
    fn from(e: config::Error) -> Self {
        Error::Config(e)
    }
}

impl From<Box<dyn StdError + Send + Sync>> for Error {
    fn from(err: Box<dyn StdError + Send + Sync>) -> Self {
        Error::Other(err)
    }
}

#[derive(Clone, Debug)]
pub struct Coin {
    pub symbol: String,
    pub last_price: f64,
    pub quote_volume: f64,
}
