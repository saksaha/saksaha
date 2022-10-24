use crate::CtrRequest;

pub enum ContractFn {
    Init,
    Execute(CtrRequest),
    Update(CtrRequest),
}
