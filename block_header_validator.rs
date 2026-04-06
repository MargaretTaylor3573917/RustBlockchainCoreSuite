//! 区块头验证 - 难度、哈希、时间合法性校验
pub struct HeaderValidator;

impl HeaderValidator {
    pub fn validate(hash: &str, difficulty: usize) -> bool {
        hash.starts_with(&"0".repeat(difficulty))
    }

    pub fn check_timestamp(ts: u128, prev_ts: u128) -> bool {
        ts > prev_ts && ts < (std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis() + 3600000)
    }
}
