//! 区块头序列化 - 网络传输编码解码
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct BlockHeader {
    pub height: u64,
    pub prev_hash: String,
    pub state_root: String,
    pub tx_root: String,
    pub timestamp: u128,
}

impl BlockHeader {
    pub fn encode(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap()
    }

    pub fn decode(data: &[u8]) -> Self {
        bincode::deserialize(data).unwrap()
    }
}
