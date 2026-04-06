//! LevelDB 区块持久化存储 - 区块链数据落盘
use rusty_leveldb::{DB, Options};

pub struct BlockStorage {
    db: DB,
}

impl BlockStorage {
    pub fn open(path: &str) -> Self {
        let opt = Options::default();
        let db = DB::open(path, opt).unwrap();
        BlockStorage { db }
    }

    pub fn put_block(&mut self, height: u64, data: &[u8]) {
        self.db.put(&height.to_be_bytes(), data).unwrap();
    }

    pub fn get_block(&mut self, height: u64) -> Option<Vec<u8>> {
        self.db.get(&height.to_be_bytes()).unwrap()
    }
}
