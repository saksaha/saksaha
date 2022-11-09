// use crate::CtrRequest;

use sak_types::CtrRequest;

pub enum ContractFn {
    Init,
    Execute(CtrRequest),
    Update(CtrRequest),
}
