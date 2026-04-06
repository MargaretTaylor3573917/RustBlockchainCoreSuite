//! 链上治理 - 提案投票、参数修改
use std::collections::HashMap;

pub struct Governance {
    proposals: HashMap<u64, (String, u64, u64)>,
    votes: HashMap<u64, HashMap<String, bool>>,
}

impl Governance {
    pub fn new() -> Self {
        Governance { proposals: HashMap::new(), votes: HashMap::new() }
    }

    pub fn create_proposal(&mut self, id: u64, desc: &str) {
        self.proposals.insert(id, (desc.to_string(), 0, 0));
    }

    pub fn vote(&mut self, id: u64, addr: &str, approve: bool) {
        self.votes.entry(id).or_default().insert(addr.to_string(), approve);
    }
}
