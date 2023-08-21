use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::address::Address;

// Account holds an Account's information
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct EsdtBalance {
    pub token_identifier: String,
    pub balance: String,
    pub nonce: Option<u64>,
    pub name: Option<String>,
    pub attributes: Option<String>,
    pub creator: Option<Address>,
    pub royalties: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct EsdtBalanceData {
    pub esdts: HashMap<String, EsdtBalance>,
}

// EsdtBalanceResponse holds the esdt balance endpoint response
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct EsdtBalanceResponse {
    pub data: Option<EsdtBalanceData>,
    pub error: String,
    pub code: String,
}
