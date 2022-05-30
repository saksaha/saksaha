use crate::{
    v1::{
        net::Connection,
        ops::{
            whoareyou::{self, WhoAreYou},
            Msg,
        },
    },
    Addr, AddrTable, BoxedError,
};
use chrono::{Duration, Utc};
use colored::Colorize;
use logger::{tdebug, terr, twarn};
use p2p_addr::{AddrStatus, KnownAddr};
use p2p_identity::Identity;
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::{RwLock, Semaphore};

pub(super) struct Handler {
    pub(crate) conn_semaphore: Arc<Semaphore>,
}

impl Handler {
    pub(super) async fn run(
        &self,
        msg: Msg,
        socket_addr: SocketAddr,
        udp_conn: Arc<Connection>,
        identity: Arc<Identity>,
        addr_table: Arc<AddrTable>,
        addr_expire_duration: Duration,
    ) -> Result<(), BoxedError> {
        match msg {
            Msg::WhoAreYouSyn(way_syn) => Ok(whoareyou::recv_who_are_you(
                socket_addr,
                udp_conn,
                way_syn,
                identity,
                addr_table,
            )
            .await?),
            Msg::WhoAreYouAck(way_ack) => {
                Ok(whoareyou::handle_who_are_you_ack(
                    way_ack,
                    socket_addr,
                    udp_conn,
                    identity,
                    addr_table,
                )
                .await?)
            }
        }
    }
}

impl Drop for Handler {
    fn drop(&mut self) {
        self.conn_semaphore.add_permits(1);
    }
}
