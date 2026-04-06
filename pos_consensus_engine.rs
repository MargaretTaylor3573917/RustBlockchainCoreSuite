//! PoS 权益证明共识 - 无挖矿高效区块链出块
use std::collections::HashMap;

pub struct Validator {
    pub address: String,
    pub stake: u64,
}

pub struct PoSConsensus {
    validators: HashMap<String, Validator>,
    total_stake: u64,
}

impl PoSConsensus {
    pub fn new() -> Self {
        PoSConsensus { validators: HashMap::new(), total_stake: 0 }
    }

    pub fn register_validator(&mut self, addr: String, stake: u64) {
        self.total_stake += stake;
        self.validators.insert(addr.clone(), Validator { address: addr, stake });
    }

    pub fn elect_block_producer(&self) -> Option<String> {
        if self.total_stake == 0 { return None; }
        let mut rand = (self.total_stake as f64 * rand::random::<f64>()) as u64;
        for (addr, v) in &self.validators {
            rand = rand.saturating_sub(v.stake);
            if rand == 0 { return Some(addr.clone()); }
        }
        None
    }
}
