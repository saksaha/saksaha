use crate::SakMRS;
use sak_contract_std::Storage;

impl SakMRS {
    pub fn api_fn() {}

    // pub async fn update(&self, request: CtrRequest) -> Result<Vec<u8>, MachineError> {
    //     let ctr_wasm = self
    //         .ledger_db
    //         .get_ctr_data_by_ctr_addr(self.ctr_addr)
    //         .await?
    //         .ok_or("ctr data (wasm) should exist")?;

    //     let ctr_state = self
    //         .ledger_db
    //         .get_ctr_state(ctr_addr)?
    //         .ok_or("ctr state should exist")?;

    //     let ctr_fn = ContractFn::Execute(request, ctr_state);

    //     let receipt = self.vm.invoke(ctr_wasm, ctr_fn)?;

    //     let state = receipt
    //         .updated_storage
    //         .ok_or("State needs to be updated after execution")?;

    //     Ok(state)
    }
}
