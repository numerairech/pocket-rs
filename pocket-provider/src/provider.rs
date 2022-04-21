use reqwest::{Client, ClientBuilder};
use std::time::Duration;
use thiserror::Error;

use crate::constants::DEFAULT_TIMEOUT;
use crate::routes::V1RpcRoutes;
use crate::types::QueryHeightResponse;

#[derive(Clone, Debug)]
pub struct PocketProvider {
    client: Client,
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
            rpc_url: cfg.rpc_url,
        }
    }

    pub async fn get_block_height(&self) -> Result<QueryHeightResponse, PocketProviderError> {
        let url = V1RpcRoutes::V1QueryHeight.url(&self.rpc_url);

        let res = self.client.post(url).send().await?;
        let text = res.text().await?;

        let resp: QueryHeightResponse = serde_json::from_str(&text)?;

        Ok(resp)
    }
}

#[derive(Clone, Debug)]
pub struct PocketProviderConfig {
    pub dispatchers: Vec<String>,
    pub rpc_url: String,
    pub timeout: Option<u64>,
}

#[derive(Debug, Error)]
pub enum PocketProviderError {
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
