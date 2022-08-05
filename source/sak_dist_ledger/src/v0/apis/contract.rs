use crate::DistLedgerApis;
use crate::LedgerError;
use log::info;
use log::warn;
use sak_contract_std::Request;
use sak_types::CtrAddr;
use sak_vm::CtrFn;

impl DistLedgerApis {
    pub async fn query_ctr(
        &self,
        ctr_addr: &CtrAddr,
        request: Request,
    ) -> Result<Vec<u8>, LedgerError> {
        let ctr_wasm = self
            .ledger_db
            .schema
            .get_ctr_data_by_ctr_addr(ctr_addr)
            .await?
            .ok_or("ctr data (wasm) should exist")?;

        println!("power00, ctr_addr: {}", ctr_addr);

        let ctr_state = self
            .ledger_db
            .schema
            .get_ctr_state(ctr_addr)?
            .ok_or("ctr state should exist")?;

        let ctr_fn = CtrFn::Query(request, ctr_state);

        println!("ctr_fn : {:?}", ctr_fn);
        println!("ctr_fn, ctr_addr : {:?}", ctr_addr);

        let ret = self.vm.invoke(ctr_wasm, ctr_fn)?;

        // if &ret[0..4] == "$$__" {
        //     return Err(format!("{}", &ret[4..]).into());
        // }

        info!(
            "invoke query ctr result : {:?}",
            String::from_utf8(ret.clone())
        );

        Ok(ret)
    }

    pub async fn execute_ctr(
        &self,
        ctr_addr: &CtrAddr,
        request: Request,
    ) -> Result<Vec<u8>, LedgerError> {
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

        {
            println!("[+] Execute Contract");
            println!(
                "[-] request.args: {:?}",
                String::from_utf8(request.args.clone()).unwrap()
            );
            println!(
                "[-] ctr_state: {:?}",
                String::from_utf8(ctr_state.clone())
            );
        }

        let ctr_fn = CtrFn::Execute(request, ctr_state);

        let ret = self.vm.invoke(ctr_wasm, ctr_fn)?;

        // if &ret[0..4] == "$$__" {
        //     let a: String = serde_json::from_slice(&ret)?;

        //     return Err(format!("Some error {}", &ret[4..]).into());
        // }

        info!("invoke execute ctr result, ctr_state: {:?}", ret);

        Ok(ret)
    }
}
