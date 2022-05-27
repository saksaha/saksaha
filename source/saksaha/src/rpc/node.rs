use crate::{machine::Machine, p2p::P2PMonitor};
use std::sync::Arc;

pub(crate) struct Node {
    pub(super) machine: Arc<Machine>,
    pub(super) p2p_monitor: Arc<P2PMonitor>,
}
