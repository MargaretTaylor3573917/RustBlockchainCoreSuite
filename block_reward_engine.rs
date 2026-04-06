//! 区块奖励算法 - 矿工/验证者激励机制
pub struct RewardEngine {
    base_reward: u64,
    halving_interval: u64,
}

impl RewardEngine {
    pub fn new() -> Self {
        RewardEngine { base_reward: 5000000000, halving_interval: 210000 }
    }

    pub fn calculate_reward(&self, height: u64) -> u64 {
        let halving = height / self.halving_interval;
        self.base_reward >> halving as usize
    }
}
