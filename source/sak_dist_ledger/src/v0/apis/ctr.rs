use crate::DistLedger;
use crate::LedgerError;
use sak_contract_std::Request;
use sak_vm::CtrFn;

impl DistLedger {
    pub async fn query_ctr(
        &self,
        ctr_addr: &String,
        request: Request,
    ) -> Result<String, LedgerError> {
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

        let ret = self.vm.invoke(ctr_wasm, ctr_fn)?;

        println!("invoke query ctr, ctr_state: {:?}", ret);

        Ok(ret)
    }

    pub async fn execute_ctr(
        &self,
        ctr_addr: &String,
        request: Request,
    ) -> Result<String, LedgerError> {
        let ctr_wasm = self
            .ledger_db
            .get_ctr_data_by_ctr_addr(ctr_addr)
            .await?
            .ok_or("ctr data (wasm) should exist")?;

        let ctr_state = self
            .ledger_db
            .get_ctr_state(ctr_addr)?
            .ok_or("ctr state should exist")?;

        let ctr_fn = CtrFn::Execute(request, ctr_state);

        let ret = self.vm.invoke(ctr_wasm, ctr_fn)?;

        println!("invoke execute ctr, ctr_state: {:?}", ret);

        Ok(ret)
    }
}
