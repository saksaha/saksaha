use crate::{CtrRequest, Storage};
use sak_store_interface::StoreAccess;
use std::sync::Arc;

pub enum ContractFn {
    Init,
    Query(CtrRequest, Storage),
    Execute(CtrRequest, Storage),
}
