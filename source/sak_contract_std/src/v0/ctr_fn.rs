use std::sync::Arc;

pub enum ContractFn {
    Init(Arc<StoreAccessor>),
    Query(CtrRequest, Storage, Arc<StoreAccessor>),
    Execute(CtrRequest, Storage, Arc<StoreAccessor>),
}
