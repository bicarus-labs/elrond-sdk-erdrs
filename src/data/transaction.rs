use std::collections::HashMap;

use super::{address::Address, vm::CallType};
use serde::{Deserialize, Serialize};

// Transaction holds the fields of a transaction to be broadcasted to the network
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Transaction {
    pub nonce: u64,
    pub value: String,
    pub receiver: Address,
    pub sender: Address,
    pub gas_price: u64,
    pub gas_limit: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
    #[serde(rename = "chainID")]
    pub chain_id: String,
    pub version: u32,
    #[serde(skip_serializing_if = "is_zero")]
    pub options: u32,
}

/// This is only used for serialize
#[allow(clippy::trivially_copy_pass_by_ref)]
fn is_zero(num: &u32) -> bool {
    *num == 0
}

// TxCostResponseData follows the format of the data field of a transaction cost request
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct TxCostResponseData {
    pub tx_gas_units: u64,
    pub return_message: String,
}

// ResponseTxCost defines a response from the node holding the transaction cost
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ResponseTxCost {
    pub data: Option<TxCostResponseData>,
    pub error: String,
    pub code: String,
}

// TransactionOnNetwork holds a transaction's info entry in a hyper block
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct TransactionOnNetwork {
    #[serde(rename = "type")]
    pub kind: String,
    pub processing_type_on_source: String,
    pub processing_type_on_destination: String,
    pub hash: Option<String>,
    pub nonce: u64,
    pub round: u64,
    pub epoch: u64,
    pub value: String,
    pub receiver: Address,
    pub sender: Address,
    pub gas_price: u64,
    pub gas_limit: Option<u64>,
    pub signature: Option<String>,
    pub source_shard: u32,
    pub destination_shard: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_nonce: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_hash: Option<String>,
    pub notarized_at_source_in_meta_nonce: Option<u64>,
    #[serde(rename = "NotarizedAtSourceInMetaHash")]
    pub notarized_at_source_in_meta_hash: Option<String>,
    pub notarized_at_destination_in_meta_nonce: Option<u64>,
    pub notarized_at_destination_in_meta_hash: Option<String>,
    pub miniblock_type: String,
    pub miniblock_hash: String,
    pub timestamp: Option<u64>,
    pub data: Option<String>,
    pub status: String,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub tokens: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub esdt_values: Vec<String>,
    pub hyperblock_nonce: Option<u64>,
    pub hyperblock_hash: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub smart_contract_results: Vec<ApiSmartContractResult>,
    pub logs: Option<ApiLogs>,
    pub operation: String,
    pub function: Option<String>,
    pub initially_paid_fee: Option<String>,
}

// Events represents the events generated by a transaction with changed fields' types in order to make it friendly for API's json
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Events {
    pub address: Address,
    pub identifier: String,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub topics: Vec<String>,
    pub data: Option<String>,
}

// ApiLogs represents logs with changed fields' types in order to make it friendly for API's json
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ApiLogs {
    pub address: Address,
    pub events: Vec<Events>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiSmartContractResult {
    pub hash: String,
    pub nonce: u64,
    pub value: u64,
    pub receiver: Address,
    pub sender: Address,
    pub data: String,
    pub prev_tx_hash: String,
    pub original_tx_hash: String,
    pub gas_limit: u64,
    pub gas_price: u64,
    pub call_type: CallType,
    pub relayer_address: Option<String>,
    pub relayed_value: Option<String>,
    pub code: Option<String>,
    pub code_metadata: Option<String>,
    pub return_message: Option<String>,
    pub original_sender: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct TransactionInfoData {
    pub transaction: TransactionOnNetwork,
}

// TransactionInfo holds a transaction info response from the network
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct TransactionInfo {
    pub error: String,
    pub code: String,
    pub data: Option<TransactionInfoData>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct TransactionStatusData {
    pub status: String,
}

// TransactionStatus holds a transaction's status response from the network
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct TransactionStatus {
    pub error: String,
    pub code: String,
    pub data: Option<TransactionStatusData>,
}

// ArgCreateTransaction will hold the transaction fields
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct ArgCreateTransaction {
    pub nonce: u64,
    pub value: String,
    pub rcv_addr: Address,
    pub snd_addr: Address,
    pub gas_price: u64,
    pub gas_limit: u64,
    pub data: Option<String>,
    pub signature: String,
    pub chain_id: String,
    pub version: u32,
    pub options: u32,
    pub available_balance: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct SendTransactionData {
    pub tx_hash: String,
}

// SendTransactionResponse holds the response received from the network when broadcasting a transaction
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct SendTransactionResponse {
    pub error: String,
    pub code: String,
    pub data: Option<SendTransactionData>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct SendTransactionsResponseData {
    pub num_of_sent_txs: i32,
    pub txs_hashes: HashMap<i32, String>,
}

// SendTransactionsResponse holds the response received from the network when broadcasting multiple transactions
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct SendTransactionsResponse {
    pub error: String,
    pub code: String,
    pub data: Option<SendTransactionsResponseData>,
}
