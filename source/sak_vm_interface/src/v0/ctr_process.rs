use crate::{InstanceState, InvokeReceipt, VMInterfaceError};
use async_trait::async_trait;
use sak_contract_std::{ContractFn, CtrRequest, Storage};

pub type ContractProcessor = Box<dyn ContractProcess + Send + Sync>;

#[async_trait]
pub trait ContractProcess {
    async fn invoke(
        &self,
        ctr_addr: &String,
        contract_wasm: &[u8],
        ctr_fn: ContractFn,
    ) -> Result<InvokeReceipt, VMInterfaceError>;
}
