//! 链分叉 resolution - 最长链/最重链选择
pub enum ForkRule { LongestChain, HeaviestChain }

pub struct ForkResolver {
    rule: ForkRule,
}

impl ForkResolver {
    pub fn new(rule: ForkRule) -> Self {
        ForkResolver { rule }
    }

    pub fn choose_chain(&self, chains: &[(u64, u64)]) -> usize {
        match self.rule {
            ForkRule::LongestChain => chains.iter().enumerate()
                .max_by_key(|(_, (h, _))| *h).map(|(i, _)| i).unwrap_or(0),
            _ => 0,
        }
    }
}
