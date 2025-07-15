use crypto_scanner::events::cex::{binance, bybit, gate, kucoin, mexc};

#[tokio::test]
async fn test_ticker_binance() {
    let client = reqwest::Client::new();
    let binance = binance::Binance::new(client);

    let res = binance.get_ticker().await;

    assert!(res.is_ok(), "Expected Ok, got {:?}", res);

    let data = res.unwrap();

    assert!(!data.is_empty(), "Expected not empty")
}

#[tokio::test]
async fn test_ticker_bybit() {
    let client = reqwest::Client::new();
    let bybit = bybit::Bybit::new(client);

    let res = bybit.get_ticker().await;

    assert!(res.is_ok(), "Expected Ok, got {:?}", res);

    let data = res.unwrap();

    assert!(!data.is_empty(), "Expected not empty")
}

#[tokio::test]
async fn test_ticker_gate() {
    let client = reqwest::Client::new();
    let gate = gate::Gate::new(client);

    let res = gate.get_ticker().await;

    assert!(res.is_ok(), "Expected Ok, got {:?}", res);

    let data = res.unwrap();

    assert!(!data.is_empty(), "Expected not empty")
}

#[tokio::test]
async fn test_ticker_kucoin() {
    let client = reqwest::Client::new();
    let kucoin = kucoin::Kucoin::new(client);

    let res = kucoin.get_ticker().await;

    assert!(res.is_ok(), "Expected Ok, got {:?}", res);

    let data = res.unwrap();

    assert!(!data.is_empty(), "Expected not empty")
}

#[tokio::test]
async fn test_ticker_mexc() {
    let client = reqwest::Client::new();
    let mexc = mexc::Mexc::new(client);

    let res = mexc.get_ticker().await;

    assert!(res.is_ok(), "Expected Ok, got {:?}", res);

    let data = res.unwrap();

    assert!(!data.is_empty(), "Expected not empty")
}