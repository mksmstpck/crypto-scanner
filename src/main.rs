use crypto_scanner::{config, handlers, services, storage};
use futures::lock::Mutex;
use log::error;
use std::collections::HashMap;
use std::process;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

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

    let hmap: Arc<Mutex<HashMap<String, storage::Pair>>> = Arc::new(Mutex::new(HashMap::new()));

    let store = storage::Storage::new(Arc::clone(&hmap));

    let services = services::Services::new(store, config.clone());

    let mut handlers = handlers::Handlers::new(config.clone(), services);

    {
        let hmap = Arc::clone(&hmap);
        tokio::spawn(async move {
            loop {
                sleep(Duration::from_secs(5)).await;

                let mut map_guard = hmap.lock().await;
                let ttl = Duration::from_secs(5 * 60);

                map_guard.retain(|key, val| {
                    let valid = val.timestamp.elapsed() < ttl;
                    if !valid {
                        println!("Removed expired: {}", key);
                    }
                    valid
                });
            }
        })
    };

    loop {
        handlers.send_pair().await;

        sleep(Duration::from_secs(1)).await;
    }
}
