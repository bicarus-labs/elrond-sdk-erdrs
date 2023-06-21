use serde::{Serialize, Deserialize};

use super::hyperblock::MiniBlock;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Block {
    pub nonce: u64,
    pub round: u64,
    pub hash: String,
    pub prev_block_hash: String,
    pub state_root_hash: String,
    pub epoch: u64,
    pub shard: u64,
    pub num_txs: u64,
    pub mini_blocks: Vec<MiniBlock>,
    pub timestamp: u64,
    pub accumulated_fees: String,
    pub developer_fees: String,
    pub status: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct BlockData {
    pub block: Block,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct BlockResponse {
    pub data: Option<BlockData>,
    pub error: String,
    pub code: String,
}
