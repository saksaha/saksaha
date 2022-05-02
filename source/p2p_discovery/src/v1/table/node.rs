use p2p_identity::addr::Addr;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug)]
pub(crate) enum Node {
    Empty,

    Valued(Arc<Mutex<Addr>>),
}
