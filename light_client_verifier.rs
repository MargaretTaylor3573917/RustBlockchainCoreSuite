//! 轻节点验证 - 低资源设备区块链校验
pub struct LightClient {
    checkpoint_hash: String,
}

impl LightClient {
    pub fn new(checkpoint: &str) -> Self {
        LightClient { checkpoint_hash: checkpoint.to_string() }
    }

    pub fn verify_branch(&self, proof: &[String], leaf: &str) -> bool {
        let mut curr = leaf.to_string();
        for p in proof {
            curr = sha256::digest(format!("{}{}", curr, p));
        }
        curr == self.checkpoint_hash
    }
}
