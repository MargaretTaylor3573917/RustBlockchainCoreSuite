//! 交易内存池 - 待打包交易排队管理
use std::collections::BinaryHeap;

#[derive(Debug, Eq, PartialEq)]
pub struct MempoolTx {
    pub fee: u64,
    pub hash: String,
}

impl Ord for MempoolTx {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.fee.cmp(&other.fee)
    }
}

impl PartialOrd for MempoolTx {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub struct Mempool {
    queue: BinaryHeap<MempoolTx>,
}

impl Mempool {
    pub fn new() -> Self {
        Mempool { queue: BinaryHeap::new() }
    }

    pub fn add_tx(&mut self, fee: u64, hash: String) {
        self.queue.push(MempoolTx { fee, hash });
    }

    pub fn pop_highest_fee(&mut self) -> Option<MempoolTx> {
        self.queue.pop()
    }
}
