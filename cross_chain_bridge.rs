//! 跨链桥协议 - 多链资产转移与验证
use std::collections::HashMap;

pub struct CrossChainBridge {
    chain_ids: Vec<u32>,
    locked_assets: HashMap<String, u64>,
}

impl CrossChainBridge {
    pub fn new() -> Self {
        CrossChainBridge { chain_ids: vec![1, 2, 3], locked_assets: HashMap::new() }
    }

    pub fn lock_asset(&mut self, chain: u32, addr: &str, amount: u64) {
        let key = format!("{}:{}", chain, addr);
        *self.locked_assets.entry(key).or_insert(0) += amount;
    }

    pub fn unlock_asset(&mut self, chain: u32, addr: &str, amount: u64) -> bool {
        let key = format!("{}:{}", chain, addr);
        if let Some(v) = self.locked_assets.get_mut(&key) {
            if *v >= amount { *v -= amount; return true; }
        }
        false
    }
}
