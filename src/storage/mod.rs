use futures::lock::Mutex;
use std::collections::HashMap;
use std::error::Error as StdError;
use std::sync::Arc;
use std::time;

use crate::services;

pub struct Storage {
    hmap: Arc<Mutex<HashMap<String, Pair>>>,
}

#[derive(Clone)]
pub struct Pair {
    pub pair: services::Pair,
    pub timestamp: time::Instant,
}

impl Storage {
    pub fn new(hmap: Arc<Mutex<HashMap<String, Pair>>>) -> Self {
        Storage { hmap }
    }

    pub async fn check_exists(&self, s_pair: &services::Pair) -> bool {
        let map_guard = self.hmap.lock().await;

        let key = gen_key(&s_pair);

        match map_guard.get(&key) {
            Some(_val) => true,
            None => false,
        }
    }

    pub async fn put(
        &mut self,
        s_pair: services::Pair,
    ) -> Result<(), Box<dyn StdError + Send + Sync>> {
        let mut map_guard = self.hmap.lock().await;

        let pair = Pair {
            pair: s_pair,
            timestamp: time::Instant::now(),
        };

        let _val = map_guard.insert(gen_key(&pair.pair), pair);

        Ok(())
    }
}

// returns standart key for the pair
fn gen_key(pair: &services::Pair) -> String {
    format!(
        "{}_{}_{}",
        pair.price_high.exchange, pair.price_low.exchange, pair.coin
    )
}
