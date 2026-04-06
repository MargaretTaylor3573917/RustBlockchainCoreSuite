//! IPFS 内容寻址 - 区块链大文件去中心化存储
use sha256::digest;
use std::collections::HashMap;

pub struct IPFSAdapter {
    store: HashMap<String, Vec<u8>>,
}

impl IPFSAdapter {
    pub fn new() -> Self {
        IPFSAdapter { store: HashMap::new() }
    }

    pub fn add_content(&mut self, data: &[u8]) -> String {
        let cid = digest(format!("IPFS_{:?}", data));
        self.store.insert(cid.clone(), data.to_vec());
        cid
    }

    pub fn get_content(&self, cid: &str) -> Option<&Vec<u8>> {
        self.store.get(cid)
    }
}
