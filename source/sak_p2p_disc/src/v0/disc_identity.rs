use std::sync::Arc;
use sak_p2p_id::Credential;

pub(crate) struct DiscIdentity {
    pub(crate) credential: Arc<Credential>,
    pub(crate) disc_port: u16,
    pub(crate) p2p_port: u16,
}
