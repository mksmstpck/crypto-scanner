use crate::{dto, events::cex};
use serde::Deserialize;
use std::error::Error;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Coin {
    pub symbol: String,
    pub last_price: String,
    pub quote_volume: String,
}

impl Coin {
    pub fn into_coin_dto(self) -> Result<dto::Coin, Box<dyn Error>> {
        let last_price = self
            .last_price
            .parse::<f64>()
            .map_err(|e| format!("last_price parse error: {}", e))?;
        let quote_volume = self
            .quote_volume
            .parse::<f64>()
            .map_err(|e| format!("quote_volume parse error: {}", e))?;

        Ok(dto::Coin {
            symbol: self.symbol,
            last_price,
            quote_volume,
        })
    }
}

pub fn into_cex_response_dto(res: Vec<Coin>) -> Result<dto::CexResponse, cex::CexError> {
    let mut res_dto = vec![dto::Coin {
        symbol: String::from(""),
        last_price: 0.0,
        quote_volume: 0.0,
    }];
    for i in res {
        let coin = match i.into_coin_dto() {
            Ok(coin) => coin,
            Err(e) => {
                println!("Error occured {}", e);
                dto::Coin {
                    symbol: String::from(""),
                    last_price: 0.0,
                    quote_volume: 0.0,
                }
            }
        };

        res_dto.push(coin);
    }

    Ok(res_dto)
}
