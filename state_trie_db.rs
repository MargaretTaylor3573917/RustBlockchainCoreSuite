//! MPT 状态树 - 以太坊账户状态存储
use sha256::digest;
use std::collections::HashMap;

pub struct StateTrie {
    nodes: HashMap<String, String>,
    root: String,
}

impl StateTrie {
    pub fn new() -> Self {
        StateTrie { nodes: HashMap::new(), root: digest("empty_trie") }
    }

    pub fn set(&mut self, key: &str, value: &str) {
        let hash = digest(format!("{}:{}", key, value));
        self.nodes.insert(key.to_string(), hash);
        self.root = digest(format!("{:?}", self.nodes));
    }

    pub fn root_hash(&self) -> &str { &self.root }
}
