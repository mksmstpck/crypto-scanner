use crate::events::{cex, models};

pub mod mexc;

pub struct Services {
    cex: cex::Cex,
}

struct FilteredCoins {
    pub market: Market,
    pub coin: Vec<models::Coin>,
}

enum Market {
    Binance,
    Bybit,
    Gate,
    Kucoin,
    Mexc,
    Okx,
}

impl Services {
    pub fn new(cex: cex::Cex) -> Services {
        Services { cex }
    }
}
