use std::collections::HashMap;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    Io(std::io::Error),
    MissingKey(String),
    ParseFloat(std::num::ParseFloatError),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Io(e) => write!(f, "Io error: {e}"),
            Error::MissingKey(e) => write!(f, "MissingKey error: {e}"),
            Error::ParseFloat(e) => write!(f, "Parse float error: {e}"),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

impl From<String> for Error {
    fn from(e: String) -> Self {
        Error::MissingKey(e)
    }
}

impl From<std::num::ParseFloatError> for Error {
    fn from(e: std::num::ParseFloatError) -> Self {
        Error::ParseFloat(e)
    }
}

pub struct Config {
    pub token: String,
    pub chat_id: String,
    pub binance_url: String,
    pub bybit_url: String,
    pub gate_url: String,
    pub kucoin_url: String,
    pub mexc_url: String,
    pub min_pair_ratio: f64,
    pub max_pair_ratio: f64,
    pub coingecko_bulk_data: String,
    pub coingecko_coin: String,
    pub coingecko_base: String,
}

fn get_required_string(
    env_variables: &HashMap<String, String>,
    key: &str,
) -> Result<String, Error> {
    env_variables
        .get(key)
        .cloned()
        .ok_or_else(|| Error::MissingKey(key.to_string()))
}

pub fn read_config(path: &str) -> Result<Config, Error> {
    let env_variables = env_file_reader::read_file(path)?;

    let config = Config {
        token: get_required_string(&env_variables, "TOKEN")?,
        chat_id: get_required_string(&env_variables, "CHAT_ID")?,
        binance_url: get_required_string(&env_variables, "BINANCE_URL")?,
        bybit_url: get_required_string(&env_variables, "BYBIT_URL")?,
        gate_url: get_required_string(&env_variables, "GATE_URL")?,
        kucoin_url: get_required_string(&env_variables, "KUCOIN_URL")?,
        mexc_url: get_required_string(&env_variables, "MEXC_URL")?,
        coingecko_bulk_data: get_required_string(&env_variables, "COINGECKO_BULK_DATA")?,
        coingecko_coin: get_required_string(&env_variables, "COINGECKO_COIN")?,
        coingecko_base: get_required_string(&env_variables, "COINGECKO_BASE")?,

        min_pair_ratio: env_variables
            .get("MIN_PAIR_RATIO")
            .ok_or_else(|| Error::MissingKey("MIN_PAIR_RATIO".to_string()))?
            .parse::<f64>()?,

        max_pair_ratio: env_variables
            .get("MAX_PAIR_RATIO")
            .ok_or_else(|| Error::MissingKey("MAX_PAIR_RATIO".to_string()))?
            .parse::<f64>()?,
    };

    Ok(config)
}
