//! UTXO 交易模型 - 比特币风格未花费交易输出
use sha256::digest;

#[derive(Debug, Clone)]
pub struct UTXO {
    pub txid: String,
    pub index: u32,
    pub amount: u64,
    pub owner_pubkey: Vec<u8>,
}

#[derive(Debug)]
pub struct Transaction {
    pub inputs: Vec<UTXO>,
    pub outputs: Vec<UTXO>,
    pub signature: Vec<u8>,
}

impl Transaction {
    pub fn tx_hash(&self) -> String {
        let data = format!("{:?}{:?}", self.inputs, self.outputs);
        digest(data)
    }

    pub fn is_valid(&self) -> bool {
        let input_sum: u64 = self.inputs.iter().map(|u| u.amount).sum();
        let output_sum: u64 = self.outputs.iter().map(|u| u.amount).sum();
        input_sum >= output_sum
    }
}
