#[derive(Clone, Debug)]
pub struct Coin {
    pub symbol: String,
    pub last_price: f64,
    pub quote_volume: f64,
}

pub type CexResponse = Vec<Coin>;
