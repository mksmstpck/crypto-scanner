use crate::events::{models};
use crate::services::Services;

impl Services {
    pub async fn get_ticker_mexc(self) {
        let res = match self.cex.mexc.get_ticker().await {
            Ok(val) => val,
            Err(e) => {
                println!("{}", e);
                models::Response::default()
            }
        };

        let mut filtered_coins: Vec<models::Coin> = vec![];

        for i in res {
            if let Ok(volume) = i.quote_volume.parse::<f64>() {
                if volume > 20_000.0 && volume < 1_000_000.0 {
                    filtered_coins.push(i);
                }
            }
        }

        println!("{:#?}", filtered_coins);
    }
}
