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

pub struct P {
    // pub validator_ctr_addr: String,
    // pub identity: Arc<Identity>,
}

#[async_trait::async_trait]
impl Consensus for P {
    async fn do_consensus(
        &self,
        dist_ledger: &SakLedger,
        tx_candidates: Vec<TxCandidate>,
    ) -> Result<BlockCandidate, ConsensusError> {
        // let request = CtrRequest {
        //     ctr_addr: self.validator_ctr_addr.to_string(),
        //     req_type: "get_validator".to_string(),
        //     args: vec![],
        //     ctr_call_type: CtrCallType::Query,
        // };

        // let validator = match dist_ledger
        //     .execute_ctr(
        //         // &self.validator_ctr_addr,
        //         request,
        //     )
        //     .await
        // {
        //     Ok(v) => v,
        //     Err(err) => {
        //         return Err(format!("Error retrieving a validator, err: {}", err).into());
        //     }
        // };

        // let validator_str: String = String::from_utf8(validator)?;

        // if self.identity.credential.public_key_str == validator_str {
        //     let bc = BlockCandidate {
        //         validator_sig: String::from("1"),
        //         tx_candidates,
        //         witness_sigs: vec![],
        //         created_at: String::from("1"),
        //     };

        //     return Ok(bc);
        // }

        return Err("Not a valid validator".into());
    }
}

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

        // let s: Box<dyn Tr + Send + Sync> = {
        //     let s = S {};
        //     Box::new(s)
        // };

        let ctr_addr = req.ctr_addr.to_string();

        let ctr_fn = ContractFn::Execute(req);

        let bc = self.consensus.do_consensus(self, vec![]).await?;
        // let _receipt = self.s.a().await;
        // receipt.result;

        // let receipt = self
        //     .contract_processor
        //     .invoke(&ctr_addr, &ctr_wasm, ctr_fn)
        //     .await?;
        // // let a = receipt.await;
        // // let r = a.unwrap();

        // let result = receipt.result;
        // let result = receipt;
        let result = vec![];

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

        // let receipt = self
        //     .contract_processor
        //     .invoke(&ctr_addr, &ctr_wasm, ctr_fn)
        //     .await?;

        // let state = receipt
        //     .updated_storage
        //     .ok_or("State needs to be updated after execution")?;

        let state = vec![];

        Ok(state)
    }
}
