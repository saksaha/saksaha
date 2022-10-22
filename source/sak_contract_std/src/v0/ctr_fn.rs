use crate::{CtrRequest, CtrRequestData, Storage};
use std::sync::Arc;

pub enum ContractFn {
    Init,
    Execute(CtrRequest),
    Update(CtrRequest),
}
