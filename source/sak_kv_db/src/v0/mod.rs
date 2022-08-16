pub mod database;
mod utils;

pub use database::*;
pub use rocksdb::{
    BoundColumnFamily, ColumnFamily, ColumnFamilyDescriptor,
    DBIteratorWithThreadMode, DBRawIteratorWithThreadMode, DBWithThreadMode,
    Direction, IteratorMode, MultiThreaded, Options, SingleThreaded,
    ThreadMode, WriteBatch, DB,
};
pub use utils::*;

pub(crate) type KvDBError = Box<dyn std::error::Error + Send + Sync>;
