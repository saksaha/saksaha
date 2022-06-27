use crate::DistLedger;
use crate::{Consensus, LedgerError};
use log::warn;
use sak_contract_std::Request;
use sak_types::{Block, BlockCandidate, Tx};
use sak_vm::FnType;
use std::{collections::HashMap, sync::Arc};

impl DistLedger {
    // pub async fn query_contract(
    //     &self,
    //     ctr_addr: &String,
    //     fn_type: FnType,
    //     request: Request,
    // ) -> Result<&[u8], LedgerError> {
    //     let ctr_wasm = self
    //         .ledger_db
    //         .get_ctr_data_by_ctr_addr(ctr_addr)
    //         .await?
    //         .ok_or("ctr data (wasm) should exist")?;

    //     let mut storage: HashMap<String, String> = HashMap::with_capacity(10);

    //     storage.insert(
    //         "validators".to_string(),
    //         serde_json::to_string(&vec![String::from(
    //             "\
    //         046885b904a8b8cdd17cc40078ed11421\
    //         4586f197a664d6aa33d4b46cc3b712afc\
    //         def3d4d808bc7843beaea9e1a4c5ddeea\
    //         47cbd27ea1af5ca13719a2f42c39167\
    //         ",
    //         )])
    //         .unwrap()
    //         .to_string(),
    //     );

    //     let ret = match self.vm.exec(ctr_wasm, fn_type, request, storage) {
    //         Ok(ret) => ret,
    //         Err(err) => return Err(err),
    //     };

    //     println!("returned!!!: {}", ret);

    //     Ok(&[])
    // }

    pub async fn exec_ctr(
        &self,
        ctr_addr: &String,
        fn_type: FnType,
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

        println!("exec ctr, ctr_state: {:?}", ctr_state);

        // let mut storage: HashMap<String, String> = HashMap::with_capacity(10);

        // storage.insert(
        //     "validators".to_string(),
        //     serde_json::to_string(&vec![String::from(
        //         "\
        //     046885b904a8b8cdd17cc40078ed11421\
        //     4586f197a664d6aa33d4b46cc3b712afc\
        //     def3d4d808bc7843beaea9e1a4c5ddeea\
        //     47cbd27ea1af5ca13719a2f42c39167\
        //     ",
        //     )])
        //     .unwrap()
        //     .to_string(),
        // );

        // let ret = match self.vm.exec(ctr_wasm, fn_type, request, storage) {
        //     Ok(ret) => ret,
        //     Err(err) => return Err(err),
        // };

        // println!("returned!!!: {}", ret);

        Ok(&[])
    }
}
