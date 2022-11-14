use std::sync::Arc;

use crate::{InvokeReceipt, VMInterfaceError};
use sak_contract_std::ContractFn;
use sak_store_interface::LedgerAccessor;

pub type ContractProcessor = Box<dyn ContractProcess + Send + Sync>;

// #[async_trait]
pub trait ContractProcess {
    // async fn invoke(
    //     &self,
    //     ctr_addr: &String,
    //     contract_wasm: &[u8],
    //     ctr_fn: ContractFn,
    // ) -> Result<InvokeReceipt, VMInterfaceError>;

    fn invoke(
        &self,
        ctr_addr: &String,
        contract_wasm: &[u8],
        ctr_fn: ContractFn,
    ) -> Result<InvokeReceipt, VMInterfaceError>;

    fn run(&self, ledger: Option<Arc<LedgerAccessor>>);
}
