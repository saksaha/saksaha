use super::{consensus::Pos, genesis::GenesisBlock};
use crate::{fs, SaksahaError};
use sak_machine::{Consensus, SakMachine, SakMachineArgs};
use sak_p2p_id::Identity;
use sak_proof::CoinProof;
use std::sync::Arc;

pub(crate) struct Ledger {
    pub(crate) dist_ledger: SakMachine,
}

impl Ledger {
    pub(crate) async fn init(
        public_key: &String,
        tx_sync_interval: Option<u64>,
        genesis_block: Option<GenesisBlock>,
        block_sync_interval: Option<u64>,
        identity: Arc<Identity>,
    ) -> Result<Self, SaksahaError> {
        let (gen_block_candidate, consensus, mrs_ctr_addr) = {
            let genesis_block = match genesis_block {
                Some(b) => b,
                None => GenesisBlock::create()?,
            };

            let validator_ctr_addr = genesis_block.get_validator_ctr_addr();

            let consensus: Box<dyn Consensus + Send + Sync> = {
                let c = Pos {
                    validator_ctr_addr,
                    identity,
                };

                Box::new(c)
            };

            (genesis_block.block_candidate, consensus, mrs_ctr_addr)
        };

        let ledger_path = {
            let acc_dir = fs::acc_dir(public_key)?;
            acc_dir.join("ledger")
        };

        let mrs_path = {
            let acc_dir = fs::acc_dir(public_key)?;
            acc_dir.join("mrs")
        };

        let dist_ledger_args = SakMachineArgs {
            tx_sync_interval,
            genesis_block: Some(gen_block_candidate),
            consensus,
            block_sync_interval,
            ledger_path,
            mrs_path,
        };

        let dist_ledger = {
            let d = SakMachine::init(dist_ledger_args).await?;

            d
        };

        let blockchain = Ledger { dist_ledger };

        Ok(blockchain)
    }

    pub async fn run(&self) {
        self.dist_ledger.run().await;
    }
}
