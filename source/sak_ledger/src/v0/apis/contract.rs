use crate::LedgerError;
use crate::SakLedger;
use sak_contract_std::ContractFn;
// use sak_contract_std::CtrRequest;
use sak_types::CtrRequest;

impl SakLedger {
    pub async fn _execute_ctr(&self, req: CtrRequest) -> Result<Vec<u8>, LedgerError> {
        let ctr_wasm = self
            .ledger_db
            .get_ctr_data_by_ctr_addr(&req.ctr_addr)
            .await?
            .ok_or("ctr data (wasm) should exist")?;

        let ctr_addr = req.ctr_addr.to_string();

        let ctr_fn = ContractFn::Execute(req);

        let receipt = self
            .contract_processor
            // .as_ref()
            // .ok_or("contract_processor should be present")?
            .invoke(&ctr_addr, &ctr_wasm, ctr_fn)?;

        let result = receipt.result;

        Ok(result)
    }

    pub async fn _update_ctr(&self, req: CtrRequest) -> Result<Vec<u8>, LedgerError> {
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
        // let receipt = self
        //     .contract_processor
        //     .as_ref()
        //     .ok_or("contract_processor should be present")?
        //     .invoke(&ctr_addr, &ctr_wasm, ctr_fn)?;

        let _ctr_state_receipt = receipt
            .updated_ctr_state
            .ok_or("State needs to be updated after execution")?;

        let _mrs_receipt = receipt
            .updated_mrs
            .ok_or("State needs to be updated after execution")?;

        // Ok(state)
        Ok(vec![222])
    }
}
