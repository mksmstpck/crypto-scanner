use crypto_scanner::{handlers, services};
use log::info;

#[tokio::main]
async fn main() {
    env_logger::init();

    let res = services::scan_all_exchanges().await.unwrap();

    let pairs = services::seek_pairs(&res).unwrap();

    for i in pairs {
        info!("{:?}", i)
    }

    let handlers = handlers::Handlers::new("8492560556:AAGLESegDDK0p8gvOqnqfc8Ga8EONKDrKGA", "-1002820842099".to_string());

    handlers.send_shit().await;
    //services::Services::new(cex).get_ticker_mexc().await;
}
