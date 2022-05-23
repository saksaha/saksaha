use super::net::{connection::UdpConn, connection2::Connection2};
use p2p_identity::identity::P2PIdentity;
use std::sync::Arc;

use super::table::Table;

pub(crate) struct DiscState {
    pub(crate) p2p_identity: Arc<P2PIdentity>,
    // pub(crate) udp_conn: Arc<UdpConn>,
    pub(crate) udp_conn: Arc<Connection2>,
    pub(crate) disc_port: u16,
    pub(crate) p2p_port: u16,
    pub(crate) table: Arc<Table>,
}
