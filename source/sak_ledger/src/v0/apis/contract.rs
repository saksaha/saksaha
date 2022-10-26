use crate::Consensus;
use crate::ConsensusError;
use crate::LedgerError;
use crate::SakLedger;
use sak_contract_std::ContractFn;
use sak_contract_std::CtrRequest;
use sak_contract_std::CtrRequestData;
use sak_contract_std::InvokeResult;
use sak_types::BlockCandidate;
use sak_types::CtrAddr;
use sak_types::TxCandidate;

impl SakLedger {
    pub async fn execute_ctr(
        &self,
        // ctr_addr: &CtrAddr,
        // data: CtrRequestData,
        req: CtrRequest,
    ) -> Result<Vec<u8>, LedgerError> {
        let ctr_wasm = self
            .ledger_db
            .get_ctr_data_by_ctr_addr(&req.ctr_addr)
            .await?
            .ok_or("ctr data (wasm) should exist")?;

        let ctr_addr = req.ctr_addr.to_string();

        let ctr_fn = ContractFn::Execute(req);

        let receipt = self
            .contract_processor
            .invoke(&ctr_addr, &ctr_wasm, ctr_fn)?;

        let result = receipt.result.clone();

        Ok(result)
    }

    pub async fn update_ctr(
        &self,
        // ctr_addr: &CtrAddr,
        // data: CtrRequestData,
        req: CtrRequest,
    ) -> Result<Vec<u8>, LedgerError> {
        let ctr_wasm = self
            .ledger_db
            .get_ctr_data_by_ctr_addr(&req.ctr_addr)
            .await?
            .ok_or("ctr data (wasm) should exist")?;

        let ctr_addr = req.ctr_addr.to_string();

        let ctr_fn = ContractFn::Execute(req);

        let receipt = self
            .contract_processor
            .invoke(&ctr_addr, &ctr_wasm, ctr_fn)?;

        let state = receipt
            .updated_storage
            .ok_or("State needs to be updated after execution")?;

        Ok(state)
    }
}
