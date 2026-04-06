//! 预言机数据喂价 - 链下数据上链
use std::collections::HashMap;

pub struct Oracle {
    prices: HashMap<String, u64>,
}

impl Oracle {
    pub fn new() -> Self {
        Oracle { prices: HashMap::new() }
    }

    pub fn update_price(&mut self, symbol: &str, price: u64) {
        self.prices.insert(symbol.to_string(), price);
    }

    pub fn get_price(&self, symbol: &str) -> Option<u64> {
        self.prices.get(symbol).copied()
    }
}
