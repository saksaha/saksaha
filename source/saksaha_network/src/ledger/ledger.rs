use super::{consensus::Pos, genesis::GenesisBlock};
use crate::{
    fs::{self, SaksahaFS},
    SaksahaError,
};
use sak_ledger::{Consensus, SakLedger, SakLedgerArgs};
use sak_p2p_id::Identity;
use std::sync::Arc;

pub(crate) struct Ledger {
    pub(crate) sak_ledger: SakLedger,
}

impl Ledger {
    pub(crate) async fn init(
        public_key: &String,
        tx_sync_interval: Option<u64>,
        genesis_block: Option<GenesisBlock>,
        block_sync_interval: Option<u64>,
        identity: Arc<Identity>,
        // contract_processor: ContractProcessor,
    ) -> Result<SakLedger, SaksahaError> {
        let (gen_block_candidate, consensus) = {
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

            (genesis_block.block_candidate, consensus)
        };

        let ledger_path = {
            let acc_dir = SaksahaFS::acc_dir(public_key)?;
            acc_dir.join("ledger")
        };

        let dist_ledger_args = SakLedgerArgs {
            tx_sync_interval,
            genesis_block: Some(gen_block_candidate),
            consensus,
            block_sync_interval,
            ledger_path,
            // contract_processor,
        };

        let sak_ledger = SakLedger::init(dist_ledger_args).await?;

        Ok(sak_ledger)
    }
}
