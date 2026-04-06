//! P2P 加密通道 - 节点通信 AES 加密
use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, KeyInit};

pub struct EncryptedChannel {
    key: [u8; 32],
}

impl EncryptedChannel {
    pub fn new(key: [u8; 32]) -> Self {
        EncryptedChannel { key }
    }

    pub fn encrypt(&self, data: &[u8]) -> Vec<u8> {
        let cipher = Aes256Gcm::new(Key::from_slice(&self.key));
        let nonce = Nonce::from_slice(b"unique_nonce_123");
        cipher.encrypt(nonce, data).unwrap()
    }

    pub fn decrypt(&self, data: &[u8]) -> Vec<u8> {
        let cipher = Aes256Gcm::new(Key::from_slice(&self.key));
        let nonce = Nonce::from_slice(b"unique_nonce_123");
        cipher.decrypt(nonce, data).unwrap()
    }
}
