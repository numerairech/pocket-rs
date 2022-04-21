use reqwest::{Client, ClientBuilder};
use std::collections::HashMap;
use std::time::Duration;
use thiserror::Error;

use crate::constants::{DEFAULT_PROVIDER_URL, DEFAULT_TIMEOUT};
use crate::routes::V1RpcRoutes;
use crate::types::*;

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

    pub async fn get_block_height(&self) -> Result<i32, PocketProviderError> {
        let url = V1RpcRoutes::QueryHeight.url(&self.rpc_url);

        let res = self.client.post(url).send().await?;
        let text = res.text().await?;

        let resp: QueryHeightResponse = serde_json::from_str(&text)?;

        Ok(resp.height)
    }

    pub async fn get_balance(&self, address: &str) -> Result<u128, PocketProviderError> {
        let url = V1RpcRoutes::QueryBalance.url(&self.rpc_url);

        let mut body = HashMap::new();
        body.insert("address", address);

        let res = self.client.post(url).json(&body).send().await?;
        let text = res.text().await?;

        let resp: QueryBalanceResponse = serde_json::from_str(&text)?;

        Ok(resp.balance)
    }

    pub async fn get_app(&self, address: &str) -> Result<Application, PocketProviderError> {
        let url = V1RpcRoutes::QueryApp.url(&self.rpc_url);

        let mut body = HashMap::new();
        body.insert("address", address);

        let res = self.client.post(url).json(&body).send().await?;
        let text = res.text().await?;

        println!("{}", text);

        let resp: Application = serde_json::from_str(&text)?;

        Ok(resp)
    }
}

#[derive(Clone, Debug)]
pub struct PocketProviderConfig {
    pub rpc_url: String,
    pub timeout: Option<u64>,
}

impl Default for PocketProviderConfig {
    fn default() -> Self {
        Self {
            rpc_url: DEFAULT_PROVIDER_URL.to_string(),
            timeout: Some(5000),
        }
    }
}

#[derive(Debug, Error)]
pub enum PocketProviderError {
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
}
