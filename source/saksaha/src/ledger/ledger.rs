use crate::db::ColumnFamilyDB;
// use super::db::DB;
// use log::{debug, error, info, warn};
// use p2p_peer::{PeerTable, PeerValue, RegisteredPeerValue};
// use std::{
//     sync::Arc,
//     time::{Duration, SystemTime},
// };

// pub(crate) struct Ledger {
//     db: Arc<DB>,
//     peer_table: Arc<PeerTable>,
// }

// impl Ledger {
//     pub fn new(peer_table: Arc<PeerTable>) -> Ledger {
//         let db = Arc::new(DB {});

//         Ledger { peer_table, db }
//     }

//     pub async fn start(&self) -> Result<(), String> {
//         info!("Ledger is started");

//         let min_interval = Duration::from_millis(2000);

//         Ok(())
//     }
// }

pub(crate) struct Ledger {
    db: ColumnFamilyDB,
}

impl Ledger {
    pub async fn init() -> Ledger {
        // db column create / check
        let db = ColumnFamilyDB::new().expect("cannot initialize ledger");

        Ledger { db }
    }
}
