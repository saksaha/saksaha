use sak_contract_std::{CtrRequest, Storage};

#[derive(Debug)]
pub enum CtrFn {
    Init,
    Query(CtrRequest, Storage),
    Execute(CtrRequest, Storage),
}
