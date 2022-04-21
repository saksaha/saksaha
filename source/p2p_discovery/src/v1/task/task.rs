use p2p_identity::addr::Addr;

#[derive(Clone, Debug)]
pub(crate) enum DiscoveryTask {
    InitiateWhoAreYou {
        // disc_state: Arc<DiscState>,
        // whoareyou_op: Arc<WhoareyouOp>,
        addr: Addr,
        // unknown_peer: UnknownPeer,
    },
}
