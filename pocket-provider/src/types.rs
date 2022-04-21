use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryHeightResponse {
    pub height: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryBalanceResponse {
    pub balance: u128,
}
