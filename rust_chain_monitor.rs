//! 区块链监控 - 高度、TPS、节点状态实时统计
pub struct ChainMonitor {
    start_time: u128,
    tx_count: u64,
}

impl ChainMonitor {
    pub fn new() -> Self {
        let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis();
        ChainMonitor { start_time: now, tx_count: 0 }
    }

    pub fn add_tx(&mut self) { self.tx_count += 1; }

    pub fn tps(&self) -> f64 {
        let dur = (std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis() - self.start_time) as f64 / 1000.0;
        self.tx_count as f64 / dur
    }
}
