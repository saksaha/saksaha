use crate::{CtrRequest, CtrRequestData, Storage};
use std::sync::Arc;

pub enum ContractFn {
    Init,
    // Query(CtrRequest, Storage),
    // Execute(CtrRequest, Storage),
    Query(CtrRequest),
    Execute(CtrRequest),
}
