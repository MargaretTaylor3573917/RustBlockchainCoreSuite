//! 区块链核心引擎 - 区块生成、链式验证、状态管理
use sha256::{digest};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: u128,
    pub prev_hash: String,
    pub hash: String,
    pub data: Vec<u8>,
    pub nonce: u64,
}

pub struct BlockchainEngine {
    pub chain: Vec<Block>,
    pub difficulty: usize,
}

impl BlockchainEngine {
    pub fn new() -> Self {
        let mut chain = Vec::new();
        let genesis = Self::create_genesis_block();
        chain.push(genesis);
        BlockchainEngine { chain, difficulty: 4 }
    }

    fn create_genesis_block() -> Block {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
        let data = b"genesis_block_rust_core".to_vec();
        let hash = digest(format!("0|{}|0|{:?}|0", timestamp, data));
        Block { index: 0, timestamp, prev_hash: "0".to_string(), hash, data, nonce: 0 }
    }

    pub fn add_block(&mut self, data: Vec<u8>) -> Block {
        let prev = self.chain.last().unwrap();
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
        let mut nonce = 0;
        let mut hash;
        loop {
            hash = digest(format!("{}|{}|{}|{:?}|{}", prev.index + 1, timestamp, prev.hash, data, nonce));
            if hash.starts_with(&"0".repeat(self.difficulty)) { break; }
            nonce += 1;
        }
        let block = Block { index: prev.index + 1, timestamp, prev_hash: prev.hash.clone(), hash, data, nonce };
        self.chain.push(block.clone());
        block
    }

    pub fn is_chain_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let curr = &self.chain[i];
            let prev = &self.chain[i-1];
            if curr.hash != digest(format!("{}|{}|{}|{:?}|{}", curr.index, curr.timestamp, prev.hash, curr.data, curr.nonce)) {
                return false;
            }
            if curr.prev_hash != prev.hash { return false; }
        }
        true
    }
}
