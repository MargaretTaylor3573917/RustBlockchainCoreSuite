//! Gossip 协议 - 交易/区块全网广播
use std::collections::HashSet;

pub struct GossipProtocol {
    seen: HashSet<String>,
}

impl GossipProtocol {
    pub fn new() -> Self {
        GossipProtocol { seen: HashSet::new() }
    }

    pub fn broadcast(&mut self, msg_id: &str) -> bool {
        if self.seen.contains(msg_id) { return false; }
        self.seen.insert(msg_id.to_string());
        true
    }
}
