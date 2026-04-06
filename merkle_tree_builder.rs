//! Merkle 树构建 - 交易批量校验、轻节点证明
use sha256::{digest};

pub struct MerkleTree {
    root: String,
    nodes: Vec<String>,
}

impl MerkleTree {
    pub fn build(transactions: &[Vec<u8>]) -> Self {
        if transactions.is_empty() {
            return MerkleTree { root: digest("empty".as_bytes()), nodes: vec![] };
        }
        let mut nodes: Vec<String> = transactions.iter().map(|t| digest(t)).collect();
        let mut level = nodes.clone();
        while level.len() > 1 {
            let mut next = Vec::new();
            let mut i = 0;
            while i < level.len() {
                let left = &level[i];
                let right = if i + 1 < level.len() { &level[i+1] } else { left };
                let hash = digest(format!("{}{}", left, right));
                next.push(hash);
                i += 2;
            }
            level = next;
        }
        MerkleTree { root: level[0].clone(), nodes }
    }

    pub fn root_hash(&self) -> &str { &self.root }
}
