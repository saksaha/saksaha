use super::{net::Connection, table::Table};
use p2p_identity::identity::P2PIdentity;
use std::sync::Arc;

pub struct DiscState {
    pub(crate) p2p_identity: Arc<P2PIdentity>,
    pub(crate) udp_conn: Arc<Connection>,
    pub(crate) disc_port: u16,
    pub(crate) p2p_port: u16,
    pub table: Arc<Table>,
}
