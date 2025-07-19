use crypto_scanner::{services};

#[tokio::main]
async fn main() {
    let res = services::scan_all_exchanges().await.unwrap();

    let pairs = services::seek_pairs(&res).unwrap();

    for i in pairs {
        println!("{:?}", i)
    }
    //services::Services::new(cex).get_ticker_mexc().await;
}
