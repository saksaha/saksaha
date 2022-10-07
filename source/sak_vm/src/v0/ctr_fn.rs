use sak_contract_std::{CtrRequest, Storage};
use sak_store_accessor::StoreAccessor;
use std::sync::Arc;

pub enum ContractFn {
    Init,
    Query(CtrRequest, Storage, Arc<StoreAccessor>),
    Execute(CtrRequest, Storage),
}
