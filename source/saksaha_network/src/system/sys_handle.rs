use crate::p2p::P2PMonitor;
use sak_machine::SakMachine;
use std::sync::Arc;

pub(crate) struct SystemHandle {
    pub(crate) machine: Arc<SakMachine>,
    pub(crate) p2p_monitor: Arc<P2PMonitor>,
}
