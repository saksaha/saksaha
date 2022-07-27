use crate::DistLedgerApis;
use crate::LedgerError;
use log::info;
use sak_contract_std::Request;
use sak_types::CtrAddr;
use sak_vm::CtrFn;

impl DistLedgerApis {
    pub async fn query_ctr(
        &self,
        ctr_addr: &CtrAddr,
        request: Request,
    ) -> Result<String, LedgerError> {
        let ctr_wasm = self
            .ledger_db
            .schema
            .get_ctr_data_by_ctr_addr(ctr_addr)
            .await?
            .ok_or("ctr data (wasm) should exist")?;

        let ctr_state = self
            .ledger_db
            .schema
            .get_ctr_state(ctr_addr)?
            .ok_or("ctr state should exist")?;

        let ctr_fn = CtrFn::Query(request, ctr_state);

        let ret = self.vm.invoke(ctr_wasm, ctr_fn)?;

        info!("invoke query ctr, ctr_state: {:?}", ret);

        Ok(ret)
    }

    pub async fn execute_ctr(
        &self,
        ctr_addr: &CtrAddr,
        request: Request,
    ) -> Result<String, LedgerError> {
        let ctr_wasm = self
            .ledger_db
            .schema
            .get_ctr_data_by_ctr_addr(ctr_addr)
            .await?
            .ok_or("ctr data (wasm) should exist")?;

        let ctr_state = self
            .ledger_db
            .schema
            .get_ctr_state(ctr_addr)?
            .ok_or("ctr state should exist")?;

        let ctr_fn = CtrFn::Execute(request, ctr_state);

        println!("123123 ctr wasm: {:?}, ctr_fn: {:?}", ctr_wasm, ctr_fn);
        let ret = self.vm.invoke(ctr_wasm, ctr_fn)?;

        info!("invoke execute ctr, ctr_state: {:?}", ret);

        Ok(ret)
    }
}
