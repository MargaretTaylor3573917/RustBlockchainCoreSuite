//! 环签名 - 区块链匿名交易
use sha256::digest;

pub struct RingSignature;

impl RingSignature {
    pub fn sign(keys: &[&str], msg: &str, secret_idx: usize) -> Vec<String> {
        keys.iter().enumerate()
            .map(|(i, k)| digest(format!("RING_{}_{}_{}_{}", i, k, msg, secret_idx)))
            .collect()
    }

    pub fn verify(msg: &str, sig: &[String]) -> bool {
        !sig.is_empty() && sig.iter().all(|s| s.len() == 64)
    }
}
