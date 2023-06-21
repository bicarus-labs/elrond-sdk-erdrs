use serde::{Deserialize, Serialize};

use super::transaction::TransactionOnNetwork;

// HyperBlock holds a hyper block's details
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HyperBlock {
    pub nonce: u64,
    pub round: u64,
    pub hash: String,
    pub prev_block_hash: String,
    pub epoch: u64,
    pub num_txs: u64,
    pub shard_blocks: Vec<ShardBlocks>,
    pub timestamp: u64,
    pub accumulated_fees: String,
    pub developer_fees: String,
    pub accumulated_fees_in_epoch: String,
    pub developer_fees_in_epoch: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MiniBlock {
    pub hash: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub processing_type: String,
    pub construction_state: Option<String>,
    pub source_shard: u64,
    pub destination_shard: u64,
    pub transactions: Vec<TransactionOnNetwork>,
    pub index_of_first_tx_processed: u64,
    pub index_of_last_tx_processed: u64,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct ShardBlocks {
    pub hash: String,
    pub nonce: u64,
    pub shard: u32,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct HyperBlockData {
    pub hyperblock: HyperBlock,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct HyperBlockResponse {
    pub data: Option<HyperBlockData>,
    pub error: String,
    pub code: String,
}
