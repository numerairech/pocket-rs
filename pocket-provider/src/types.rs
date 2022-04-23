use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryHeightResponse {
    pub height: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryBalanceResponse {
    pub balance: u128,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Application {
    pub address: String,
    pub public_key: String,
    pub jailed: bool,
    pub status: i32,
    pub chains: Vec<String>,
    // TODO: Find a way to treat this as a u128, probably using some intermediate struct
    pub staked_tokens: String,
    pub max_relays: String,
    pub unstaking_time: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Node {
    pub address: String,
    pub chains: Vec<String>,
    pub jailed: bool,
    pub public_key: String,
    pub service_url: String,
    pub status: i32,
    pub tokens: i128,
    pub unstaking_time: String,
}
