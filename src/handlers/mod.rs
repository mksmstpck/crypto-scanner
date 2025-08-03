use core::str;

use log::{error, info};
use teloxide::{self, prelude::{Requester, RequesterExt}};

pub struct Handlers {
    bot: teloxide::prelude::AutoSend<teloxide::prelude::Bot>,
    chat_id: String,
}

impl Handlers {
    pub fn new(token: &str, chat_id: String) -> Handlers {
        let bot = teloxide::prelude::Bot::new(token).auto_send();

        Handlers {
            bot,
            chat_id,
        }
    }

    pub async fn send_shit (self) {
        let m = self.bot.send_message(self.chat_id, "shit").await;

        match m {
            Ok(val) => info!("{:?}", &val),
            Err(err) => error!("{err}"),
        }
    }
}