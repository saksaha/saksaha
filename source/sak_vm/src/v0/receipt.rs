use sak_contract_std::{InvokeResult, Storage};

pub struct InvokeReceipt {
    pub gas_charged: usize,
    pub fn_type: &'static str,
    pub result: InvokeResult,
    pub updated_storage: Storage,
}
