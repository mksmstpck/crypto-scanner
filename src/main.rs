use crypto_scanner::{
    events::{cex},
    services,
};

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();

    let cex = cex::Cex::new(client);

    services::Services::new(cex).get_ticker_mexc().await;
}
