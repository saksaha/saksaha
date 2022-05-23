use crate::blockchain::Blockchain;
use p2p_discovery::Discovery;
use std::sync::Arc;

pub(crate) struct Machine {
    pub(crate) blockchain: Blockchain,
    pub(crate) machine_discovery: Arc<Discovery>,
}
