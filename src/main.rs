use crypto_scanner::{services};

#[tokio::main]
async fn main() {
    let res = services::scan_all_exchanges().await.unwrap();

    let pairs = services::seek_pairs(&res).unwrap();

    let mut pairs_filtered = vec![]; 

    for i in pairs {
        if i.spread_percents > 2.0 {
            pairs_filtered.push(i.clone());

            println!("{:?}", i)
        }
    }
    //services::Services::new(cex).get_ticker_mexc().await;
}
