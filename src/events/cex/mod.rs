use reqwest::{self};

pub mod binance;
pub mod bybit;
pub mod gate;
pub mod kucoin;
pub mod mexc;

pub struct Cex {
    pub binance:binance::Binance,
    pub bybit:bybit::Bybit,
    pub gate:gate::Gate,
    pub kucoin:kucoin::Kucoin,
    pub mexc:mexc::Mexc,

}

pub type CexResponse = Vec<models::Coin>;

impl Cex {
    pub fn new(client: reqwest::Client) -> Cex {
        Cex { 
            binance: binance::Binance::new(client.clone()),
            bybit: bybit::Bybit::new(client.clone()),
            gate: gate::Gate::new(client.clone()),
            kucoin: kucoin::Kucoin::new(client.clone()),
            mexc: mexc::Mexc::new(client.clone()),
         }
    }
}
