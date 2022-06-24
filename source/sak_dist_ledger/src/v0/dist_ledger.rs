use super::tx_pool::TxPool;
use super::DLedgerEvent;
use crate::Database;
use crate::Runtime;
use log::{error, info, warn};
use sak_contract_std::Request;
use sak_contract_std::Storage;
use sak_types::Block;
use sak_vm::FnType;
use sak_vm::VM;
use std::sync::Arc;
use tokio::sync::broadcast;
use tokio::sync::{broadcast::Sender, RwLock};

const BLOCKCHAIN_EVENT_QUEUE_CAPACITY: usize = 32;

pub struct DistLedger {
    pub(crate) database: Database,
    pub(crate) tx_pool: Arc<TxPool>,
    pub bc_event_tx: Arc<RwLock<Sender<DLedgerEvent>>>,
    vm: VM,
    runtime: Arc<Runtime>,
}

pub struct DistLedgerArgs {
    pub app_prefix: String,
    pub tx_pool_sync_interval: Option<u64>,
}

impl DistLedger {
    pub async fn init(
        blockchain_args: DistLedgerArgs,
    ) -> Result<DistLedger, String> {
        let DistLedgerArgs {
            app_prefix,
            tx_pool_sync_interval,
        } = blockchain_args;

        let database = match Database::init(&app_prefix).await {
            Ok(d) => d,
            Err(err) => {
                return Err(format!(
                    "Error initializing database, err: {}",
                    err,
                ));
            }
        };

        let vm = VM::init()?;

        let tx_pool = {
            let t = TxPool::new();

            Arc::new(t)
        };

        let bc_event_tx = {
            let (tx, _rx) = broadcast::channel(BLOCKCHAIN_EVENT_QUEUE_CAPACITY);

            Arc::new(RwLock::new(tx))
        };

        let runtime = {
            let r = Runtime::init(
                tx_pool.clone(),
                bc_event_tx.clone(),
                tx_pool_sync_interval,
            );

            Arc::new(r)
        };

        let dist_ledger = DistLedger {
            database,
            tx_pool: tx_pool.clone(),
            vm,
            bc_event_tx,
            runtime,
        };

        info!("Initialized Blockchain");

        Ok(dist_ledger)
    }

    pub async fn run(&self) {
        let runtime = self.runtime.clone();

        tokio::spawn(async move {
            runtime.run().await;
        });
    }

    pub async fn get_gen_block(&self) -> Result<Block, String> {
        self.get_block_by_height(String::from("0")).await
    }

    pub fn exec_ctr(
        &self,
        ctr_addr: String,
        fn_type: FnType,
        request: Request,
    ) {

        // let ctr = self.get_contract_state(tx_hashes)
        // self.vm.exec(
        //     contract_wasm: &[u8],
        //     fn_type: FnType,
        //     request: Request,
        //     storage: Storage,
        // )
    }
}
