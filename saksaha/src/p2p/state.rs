use std::sync::Arc;
use saksaha_p2p_identity::Identity;

pub struct HostState {
    identity: Arc<Identity>,
}
