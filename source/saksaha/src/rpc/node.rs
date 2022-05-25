use crate::{machine::Machine, p2p::P2PState};
use std::sync::Arc;

pub(crate) struct Node {
    pub(super) machine: Arc<Machine>,
    pub(super) p2p_state: Arc<P2PState>,
}
