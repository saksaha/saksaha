use crate::{
    whoareyou::{self, WhoAreYouRecvError},
    AddrTable, BoxedError, Connection, Msg,
};
use log::warn;
use p2p_identity::Identity;
use std::{net::SocketAddr, sync::Arc, time::Duration};
use tokio::sync::Semaphore;

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
        _addr_expire_duration: Duration,
    ) -> Result<(), BoxedError> {
        match msg {
            Msg::WhoAreYouSyn(way_syn) => {
                let res = whoareyou::recv_who_are_you(
                    socket_addr,
                    udp_conn,
                    way_syn,
                    identity,
                    addr_table,
                )
                .await;

                match res {
                    Ok(_) => return Ok(()),
                    Err(way_recv_err) => match way_recv_err {
                        WhoAreYouRecvError::AddrAlreadyMapped { .. } => {
                            warn!(
                                "Error receiving whoareyou, err: {}",
                                way_recv_err,
                            );
                        }
                        _ => return Err(way_recv_err.into()),
                    },
                };

                Ok(())
            }
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
