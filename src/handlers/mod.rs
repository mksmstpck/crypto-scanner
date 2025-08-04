use log::{error, info};
use teloxide::{self, prelude::{Requester, RequesterExt}};
use std::sync::Arc;
use crate::config;

pub struct Handlers {
    bot: teloxide::prelude::AutoSend<teloxide::prelude::Bot>,
    config: Arc<config::Config>,
}

impl Handlers {
    pub fn new(config: Arc<config::Config>) -> Handlers {
        let bot = teloxide::prelude::Bot::new(&config.token).auto_send();

        Handlers {
            bot,
            config: config.clone(),
        }
    }

    pub async fn send_shit (self) {
        let m = self.bot.send_message(self.config.chat_id.to_string(), "shit").await;

        match m {
            Ok(val) => info!("{:?}", &val),
            Err(err) => error!("{err}"),
        }
    }
}