use crypto_scanner::{config, handlers, services};
use log::info;
use log::error;
use std::process;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    env_logger::init();

    let config = match config::read_config("config.env") {
        Ok(c) => Arc::new(c),
        Err(e) => {
            error!("{e}");
            process::exit(1);
        }
    };

    let res = services::scan_all_exchanges(config.clone()).await.unwrap();

    let pairs = services::seek_pairs(&res).unwrap();

    for i in pairs {
        info!("{:?}", i)
    }

    let handlers = handlers::Handlers::new(config.clone());

    handlers.send_shit().await;
    //services::Services::new(cex).get_ticker_mexc().await;
}
