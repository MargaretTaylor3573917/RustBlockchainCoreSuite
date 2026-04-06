//! 交易签名批量验证 - 区块交易合法性校验
pub struct TxVerifier;

impl TxVerifier {
    pub fn batch_verify(pubkeys: &[&[u8]], msgs: &[&[u8]], sigs: &[&[u8]]) -> bool {
        for i in 0..pubkeys.len() {
            if !crate::ecdsa_secp256k1::ECDSASigner::verify(pubkeys[i], msgs[i], sigs[i]) {
                return false;
            }
        }
        true
    }
}
