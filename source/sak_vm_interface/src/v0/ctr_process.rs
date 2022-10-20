use crate::{InstanceState, InvokeReceipt, VMInterfaceError};
use sak_contract_std::{ContractFn, CtrRequest, Storage};

pub type ContractProcessor = Box<dyn ContractProcess + Send + Sync>;

pub trait ContractProcess {
    fn invoke(
        &self,
        contract_wasm: &[u8],
        ctr_fn: ContractFn,
    ) -> Result<InvokeReceipt, VMInterfaceError>;
}
