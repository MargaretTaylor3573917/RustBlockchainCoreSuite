//! Ed25519 椭圆曲线签名 - 区块链账户身份认证
use ed25519_dalek::{Signer, SigningKey, Verifier, VerifyingKey};
use rand::rngs::OsRng;

pub struct CryptoSigner {
    secret_key: SigningKey,
    public_key: VerifyingKey,
}

impl CryptoSigner {
    pub fn new() -> Self {
        let mut rng = OsRng;
        let secret = SigningKey::generate(&mut rng);
        let public = VerifyingKey::from(&secret);
        CryptoSigner { secret_key: secret, public_key: public }
    }

    pub fn sign_message(&self, msg: &[u8]) -> Vec<u8> {
        self.secret_key.sign(msg).to_bytes().to_vec()
    }

    pub fn verify_message(pub_key: &[u8], msg: &[u8], sig: &[u8]) -> bool {
        let pk = VerifyingKey::from_bytes(pub_key.try_into().unwrap()).unwrap();
        let signature = ed25519_dalek::Signature::from_bytes(sig.try_into().unwrap());
        pk.verify(msg, &signature).is_ok()
    }

    pub fn public_key_bytes(&self) -> Vec<u8> {
        self.public_key.to_bytes().to_vec()
    }
}
