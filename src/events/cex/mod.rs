use std::sync::Arc;

use reqwest::{self};

use crate::dto;

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
pub trait CexApi {
    async fn get_ticker(&self) -> Result<Vec<dto::Coin>, CexError>;
}

#[derive(Debug)]
pub enum CexError {
    Http(reqwest::Error),
    Json(serde_json::Error),
    Other(String),
}

impl std::fmt::Display for CexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CexError::Http(e) => write!(f, "HTTP error: {}", e),
            CexError::Json(e) => write!(f, "JSON error: {}", e),
            CexError::Other(e) => write!(f, "Other error: {}", e),
        }
    }
}

impl std::error::Error for CexError {}

impl From<reqwest::Error> for CexError {
    fn from(e: reqwest::Error) -> Self {
        CexError::Http(e)
    }
}

impl From<serde_json::Error> for CexError {
    fn from(e: serde_json::Error) -> Self {
        CexError::Json(e)
    }
}
