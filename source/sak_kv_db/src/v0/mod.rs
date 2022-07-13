pub mod database;
mod utils;

pub use database::*;
pub use rocksdb::{
    BoundColumnFamily, ColumnFamily, ColumnFamilyDescriptor,
    DBRawIteratorWithThreadMode, DBWithThreadMode, Direction, IteratorMode,
    Options, WriteBatch, DB,
};
pub use utils::*;

pub(crate) type KvDBError = Box<dyn std::error::Error + Send + Sync>;
