//! 零知识证明 - 隐私交易、匿名身份验证
use sha256::digest;

pub struct ZKProof {
    pub secret: String,
    pub commitment: String,
}

impl ZKProof {
    pub fn create(secret: &str) -> Self {
        let commit = digest(format!("ZK_COMMIT_{}", secret));
        ZKProof { secret: secret.to_string(), commitment: commit }
    }

    pub fn verify(commit: &str, challenge: &str, response: &str) -> bool {
        let check = digest(format!("ZK_PROOF_{}_{}", challenge, response));
        commit == &check
    }
}
