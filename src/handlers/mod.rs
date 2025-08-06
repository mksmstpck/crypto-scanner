use crate::{
    config,
    services::{self},
};
use log::{error, info};
use std::sync::Arc;
use teloxide::{
    self,
    prelude::{Requester, RequesterExt},
};

pub struct Handlers {
    bot: teloxide::prelude::AutoSend<teloxide::prelude::Bot>,
    s: services::Services,
    config: Arc<config::Config>,
}

impl Handlers {
    pub fn new(config: Arc<config::Config>, s: services::Services) -> Handlers {
        let bot = teloxide::prelude::Bot::new(&config.token).auto_send();

        Handlers {
            bot,
            config: config.clone(),
            s,
        }
    }

    pub async fn send_pair(&mut self) {
        let pairs = match self.s.seek_pairs().await {
            Ok(val) => val,
            Err(err) => {
                error!("{err}");
                vec![]
            }
        };

        for i in pairs {
            let m = self
                .bot
                .send_message(
                    self.config.chat_id.to_string(),
                    format!(
                        "Pair: \n Coin: {} \n Exchange high: {} \n Exchange low: {} \n Spread: {}%",
                        i.coin, i.price_high.exchange, i.price_low.exchange, i.spread_percents
                    ),
                )
                .await;

            match m {
                Ok(val) => info!("{:?}", &val),
                Err(err) => error!("{err}"),
            }
        }
    }
}
