use crate::LedgerError;
use crate::SakDistLedger;
use sak_contract_std::CtrRequest;
use sak_types::CtrAddr;
use sak_vm::ContractFn;

impl SakDistLedger {
    pub async fn query_ctr(
        &self,
        ctr_addr: &CtrAddr,
        request: CtrRequest,
    ) -> Result<Vec<u8>, LedgerError> {
        let ctr_wasm = self
            .ledger_db
            .get_ctr_data_by_ctr_addr(ctr_addr)
            .await?
            .ok_or("ctr data (wasm) should exist")?;

        let ctr_state = self
            .ledger_db
            .get_ctr_state(ctr_addr)?
            .ok_or("ctr state should exist")?;

        let ctr_fn = ContractFn::Query(request, ctr_state);

        let receipt = self.vm.invoke(ctr_wasm, ctr_fn)?;

        let result = receipt.result;

        Ok(result)
    }

    pub async fn execute_ctr(
        &self,
        ctr_addr: &CtrAddr,
        request: CtrRequest,
    ) -> Result<Vec<u8>, LedgerError> {
        let ctr_wasm = self
            .ledger_db
            .get_ctr_data_by_ctr_addr(ctr_addr)
            .await?
            .ok_or("ctr data (wasm) should exist")?;

        let ctr_state = self
            .ledger_db
            .get_ctr_state(ctr_addr)?
            .ok_or("ctr state should exist")?;

        let ctr_fn = ContractFn::Execute(request, ctr_state);

        let receipt = self.vm.invoke(ctr_wasm, ctr_fn)?;

        let state = receipt
            .updated_storage
            .ok_or("State needs to be updated after execution")?;

        Ok(state)
    }
}
