use crypto_scanner::services;
use log::info;

#[tokio::main]
async fn main() {
    env_logger::init();

    let res = services::scan_all_exchanges().await.unwrap();

    let pairs = services::seek_pairs(&res).unwrap();

    for i in pairs {
        info!("{:?}", i)
    }
    //services::Services::new(cex).get_ticker_mexc().await;
}
