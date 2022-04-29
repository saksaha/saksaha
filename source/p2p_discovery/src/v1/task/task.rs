use p2p_identity::addr::Addr;
use std::sync::Arc;

use crate::state::DiscState;

#[derive(Clone)]
pub(crate) enum DiscoveryTask {
    InitiateWhoAreYou {
        // disc_state: Arc<DiscState>,
        // whoareyou_op: Arc<WhoareyouOp>,
        addr: Addr,
        disc_state: Arc<DiscState>,
        // unknown_peer: UnknownPeer,
    },
}
