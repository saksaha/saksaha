use crate::{CtrRequest, CtrRequestData, Storage};
use sak_store_interface::StoreAccess;
use std::sync::Arc;

pub enum ContractFn {
    Init,
    // Query(CtrRequest, Storage),
    // Execute(CtrRequest, Storage),
    Query(CtrRequest),
    Execute(CtrRequest),
}
