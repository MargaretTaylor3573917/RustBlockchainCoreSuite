//! DPoS 委托权益证明 - 高效联盟链共识
use std::collections::HashMap;

pub struct DPoS {
    candidates: HashMap<String, u64>,
    elected: Vec<String>,
    count: usize,
}

impl DPoS {
    pub fn new(elected_count: usize) -> Self {
        DPoS { candidates: HashMap::new(), elected: Vec::new(), count: elected_count }
    }

    pub fn vote(&mut self, candidate: &str, votes: u64) {
        *self.candidates.entry(candidate.to_string()).or_insert(0) += votes;
    }

    pub fn elect(&mut self) {
        let mut list: Vec<_> = self.candidates.iter().collect();
        list.sort_by(|a,b| b.1.cmp(a.1));
        self.elected = list.into_iter().take(self.count).map(|(k,_)| k.clone()).collect();
    }
}
