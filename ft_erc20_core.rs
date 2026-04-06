//! FT ERC20 标准实现 - 同质化代币核心
use std::collections::HashMap;

pub struct ERC20 {
    total_supply: u64,
    balances: HashMap<String, u64>,
    allowances: HashMap<(String, String), u64>,
}

impl ERC20 {
    pub fn new(supply: u64) -> Self {
        let mut balances = HashMap::new();
        balances.insert("root".to_string(), supply);
        ERC20 { total_supply: supply, balances, allowances: HashMap::new() }
    }

    pub fn transfer(&mut self, from: &str, to: &str, amount: u64) -> bool {
        let f = self.balances.get_mut(from)?;
        if *f < amount { return false; }
        *f -= amount;
        *self.balances.entry(to.to_string()).or_insert(0) += amount;
        true
    }
}
