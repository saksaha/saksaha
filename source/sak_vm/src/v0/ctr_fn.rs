use sak_contract_std::{Request, Storage};

#[derive(Debug)]
pub enum CtrFn {
    Init,
    Query(Request, Storage),
    Execute(Request, Storage),
}
