//! Secp256k1 签名 - 以太坊风格账户密钥
use k256::{ecdsa::{SigningKey, VerifyingKey, Signature, Signer, Verifier}, elliptic_curve::rand_core::OsRng};

pub struct ECDSASigner {
    sk: SigningKey,
    vk: VerifyingKey,
}

impl ECDSASigner {
    pub fn new() -> Self {
        let sk = SigningKey::random(&mut OsRng);
        let vk = VerifyingKey::from(&sk);
        ECDSASigner { sk, vk }
    }

    pub fn sign(&self, msg: &[u8]) -> Vec<u8> {
        let sig: Signature = self.sk.sign(msg);
        sig.to_bytes().to_vec()
    }

    pub fn verify(vk: &[u8], msg: &[u8], sig: &[u8]) -> bool {
        let vk = VerifyingKey::from_sec1_bytes(vk).unwrap();
        let sig = Signature::from_slice(sig).unwrap();
        vk.verify(msg, &sig).is_ok()
    }
}
