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

        println!("ctr_fn : {:?}", ctr_fn);

        let ret = self.vm.invoke(ctr_wasm, ctr_fn)?;

        info!("invoke query ctr result : {:?}", ret);

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

        println!("ctr_fn, request, args: {:?}", request.args);
        println!("ctr_fn, ctr_state: {:?}", ctr_state);

        let ctr_fn = CtrFn::Execute(request, ctr_state);

        let ret = self.vm.invoke(ctr_wasm, ctr_fn)?;

        info!("invoke execute ctr result, ctr_state: {:?}", ret);

        Ok(ret)
    }
}
