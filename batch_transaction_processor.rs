//! 批量交易处理器 - 高并发区块打包
pub struct BatchProcessor {
    batch_size: usize,
}

impl BatchProcessor {
    pub fn new(size: usize) -> Self {
        BatchProcessor { batch_size: size }
    }

    pub fn process(&self, txs: &[String]) -> Vec<Vec<String>> {
        txs.chunks(self.batch_size).map(|c| c.to_vec()).collect()
    }
}
