//! 区块链同步协议 - 节点间区块数据自动对齐
use std::collections::VecDeque;

pub struct SyncProtocol {
    local_height: u64,
    remote_height: u64,
    sync_queue: VecDeque<u64>,
}

impl SyncProtocol {
    pub fn new(local: u64, remote: u64) -> Self {
        let mut queue = VecDeque::new();
        for i in local+1..=remote { queue.push_back(i); }
        SyncProtocol { local_height: local, remote_height: remote, sync_queue: queue }
    }

    pub fn next_block_height(&mut self) -> Option<u64> {
        self.sync_queue.pop_front()
    }

    pub fn is_sync_completed(&self) -> bool {
        self.sync_queue.is_empty()
    }
}
