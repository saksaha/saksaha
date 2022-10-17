use crate::{InstanceState, InvokeReceipt, VMInterfaceError};
use sak_contract_std::{ContractFn, CtrRequest, Storage};
// use wasmtime::{Instance, Memory, Store};

pub type ContractProcessor = Box<dyn ContractProcess + Send + Sync>;

pub trait ContractProcess {
    fn invoke(
        &self,
        contract_wasm: &[u8],
        ctr_fn: ContractFn,
    ) -> Result<InvokeReceipt, VMInterfaceError>;

    // fn invoke(
    //     &self,
    //     contract_wasm: impl AsRef<[u8]>,
    //     ctr_fn: ContractFn,
    // ) -> Result<InvokeReceipt, VMInterfaceError>;

    // fn invoke_init(
    //     instance: Instance,
    //     store: Store<InstanceState>,
    //     memory: Memory,
    // ) -> Result<InvokeReceipt, VMInterfaceError>;

    // fn invoke_query(
    //     instance: Instance,
    //     store: Store<InstanceState>,
    //     memory: Memory,
    //     request: CtrRequest,
    //     storage: Storage,
    // ) -> Result<InvokeReceipt, VMInterfaceError>;

    // fn invoke_execute(
    //     instance: Instance,
    //     store: Store<InstanceState>,
    //     memory: Memory,
    //     request: CtrRequest,
    //     storage: Storage,
    // ) -> Result<InvokeReceipt, VMInterfaceError>;
}
