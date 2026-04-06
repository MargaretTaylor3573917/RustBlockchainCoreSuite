//! 分片链管理 - 高吞吐区块链水平扩展
use std::collections::HashMap;

pub struct ShardManager {
    shards: HashMap<u8, Vec<String>>,
    count: u8,
}

impl ShardManager {
    pub fn new(shard_count: u8) -> Self {
        ShardManager { shards: HashMap::new(), count: shard_count }
    }

    pub fn assign_address(&self, addr: &str) -> u8 {
        let hash = sha256::digest(addr);
        hash.bytes().next().unwrap() % self.count
    }
}
