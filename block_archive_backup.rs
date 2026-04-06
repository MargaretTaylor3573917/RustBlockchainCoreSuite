//! 区块归档备份 - 历史数据冷存储
use std::fs::write;

pub struct BlockArchive;

impl BlockArchive {
    pub fn backup(height: u64, data: &[u8]) -> std::io::Result<()> {
        write(format!("archive_block_{}.dat", height), data)
    }
}
