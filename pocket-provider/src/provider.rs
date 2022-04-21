use reqwest::{Client, ClientBuilder};
use std::time::Duration;

use crate::constants::DEFAULT_TIMEOUT;

#[derive(Clone, Debug)]
pub struct PocketProvider {
    client: Client,
    dispatchers: Vec<String>,
    rpc_url: String,
}

impl PocketProvider {
    pub fn new(cfg: PocketProviderConfig) -> Self {
        let mut builder = ClientBuilder::new();
        builder = builder.timeout(Duration::from_millis(
            cfg.timeout.unwrap_or(DEFAULT_TIMEOUT),
        ));

        let client = builder.build().unwrap();

        Self {
            client,
            dispatchers: cfg.dispatchers,
            rpc_url: cfg.rpc_url,
        }
    }
}

#[derive(Clone, Debug)]
pub struct PocketProviderConfig {
    pub dispatchers: Vec<String>,
    pub rpc_url: String,
    pub timeout: Option<u64>,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
