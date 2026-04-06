//! NFT ERC721 标准实现 - 非同质化代币核心
use std::collections::HashMap;

pub struct ERC721 {
    owner: HashMap<u64, String>,
    balance: HashMap<String, u64>,
}

impl ERC721 {
    pub fn new() -> Self {
        ERC721 { owner: HashMap::new(), balance: HashMap::new() }
    }

    pub fn mint(&mut self, token_id: u64, to: &str) {
        self.owner.insert(token_id, to.to_string());
        *self.balance.entry(to.to_string()).or_insert(0) += 1;
    }

    pub fn owner_of(&self, token_id: u64) -> Option<&String> {
        self.owner.get(&token_id)
    }
}
