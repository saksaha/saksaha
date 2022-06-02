use crate::{machine::Machine, p2p::P2PMonitor};
use std::sync::Arc;

pub(crate) struct SystemHandle {
    pub(crate) machine: Arc<Machine>,
    pub(crate) p2p_monitor: Arc<P2PMonitor>,
}
