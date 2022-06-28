use crate::DistLedger;
use crate::{Consensus, LedgerError};
use log::warn;
use sak_contract_std::Request;
use sak_types::{Block, BlockCandidate, Tx};
use sak_vm::CtrFn;
use std::{collections::HashMap, sync::Arc};

impl DistLedger {
    pub async fn query_ctr(
        &self,
        ctr_addr: &String,
        request: Request,
    ) -> Result<&[u8], LedgerError> {
        let ctr_wasm = self
            .ledger_db
            .get_ctr_data_by_ctr_addr(ctr_addr)
            .await?
            .ok_or("ctr data (wasm) should exist")?;

        let ctr_state = self
            .ledger_db
            .get_ctr_state(ctr_addr)?
            .ok_or("ctr state should exist")?;

        let ctr_fn = CtrFn::Query(request, ctr_state);

        let ret = self.vm.exec(ctr_wasm, ctr_fn)?;

        println!("exec ctr, ctr_state: {:?}", ret);

        Ok(&[])
    }
}
