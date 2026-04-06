//! 验证者惩罚机制 - 作恶节点质押扣除
use std::collections::HashMap;

pub struct SlashingEngine {
    stakes: HashMap<String, u64>,
    penalty_pct: u8,
}

impl SlashingEngine {
    pub fn new() -> Self {
        SlashingEngine { stakes: HashMap::new(), penalty_pct: 10 }
    }

    pub fn set_stake(&mut self, addr: &str, stake: u64) {
        self.stakes.insert(addr.to_string(), stake);
    }

    pub fn slash(&mut self, addr: &str) -> u64 {
        if let Some(s) = self.stakes.get_mut(addr) {
            let penalty = *s * self.penalty_pct as u64 / 100;
            *s -= penalty;
            return penalty;
        }
        0
    }
}
