use crate::MachineError;
use crate::SakLedger;
use sak_contract_std::ContractFn;
use sak_contract_std::CtrRequest;
use sak_contract_std::CtrRequestData;
use sak_types::CtrAddr;

impl SakLedger {
    pub async fn execute_ctr(
        &self,
        // ctr_addr: &CtrAddr,
        // data: CtrRequestData,
        req: CtrRequest,
    ) -> Result<Vec<u8>, MachineError> {
        let ctr_wasm = self
            .ledger_db
            .get_ctr_data_by_ctr_addr(&req.ctr_addr)
            .await?
            .ok_or("ctr data (wasm) should exist")?;

        // let ctr_state = self
        //     .ledger_db
        //     .get_ctr_state(ctr_addr)?
        //     .ok_or("ctr state should exist")?;

        // let ctr_fn = ContractFn::Query(request, ctr_state);

        // let req = CtrRequest {
        //     ctr_addr: ctr_addr.to_string(),
        //     req_type: data.req_type,
        //     args: data.args,
        //     ctr_call_type: data.ctr_call_type,
        // };

        let ctr_addr = req.ctr_addr.to_string();

        let ctr_fn = ContractFn::Execute(req);

        let receipt = self.contract_processor.invoke(&ctr_addr, &ctr_wasm, ctr_fn);
        let a = receipt.await;

        let r = a.unwrap();

        let result = r.result;

        Ok(result)
    }

    pub async fn update_ctr(
        &self,
        // ctr_addr: &CtrAddr,
        // data: CtrRequestData,
        req: CtrRequest,
    ) -> Result<Vec<u8>, MachineError> {
        let ctr_wasm = self
            .ledger_db
            .get_ctr_data_by_ctr_addr(&req.ctr_addr)
            .await?
            .ok_or("ctr data (wasm) should exist")?;

        let ctr_addr = req.ctr_addr.to_string();

        // let ctr_state = self
        //     .ledger_db
        //     .get_ctr_state(ctr_addr)?
        //     .ok_or("ctr state should exist")?;

        // let ctr_fn = ContractFn::Execute(request, ctr_state);
        // let req = CtrRequest {
        //     ctr_addr: ctr_addr.to_string(),
        //     req_type: data.req_type,
        //     args: data.args,
        //     ctr_call_type: data.ctr_call_type,
        // };

        let ctr_fn = ContractFn::Execute(req);

        let receipt = self
            .contract_processor
            .invoke(&ctr_addr, &ctr_wasm, ctr_fn)
            .await?;

        let state = receipt
            .updated_storage
            .ok_or("State needs to be updated after execution")?;

        Ok(state)
    }
}
